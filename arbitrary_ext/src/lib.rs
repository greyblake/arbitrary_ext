use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Attribute, DeriveInput, Field, Fields, Ident};

#[proc_macro_derive(ArbitraryExt, attributes(arbitrary_ext))]
pub fn derive(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = syn::parse(input).unwrap();
    impl_arbitrary(&derive_input).into()
}


fn impl_arbitrary(input: &DeriveInput) -> TokenStream2 {
    use syn::Data::*;
    let struct_name = &input.ident;

    let inner_impl = match input.data {
        Struct(ref ds) => match ds.fields {
            Fields::Named(ref fields) => impl_arbitrary_for_struct(struct_name, &fields.named),
            _ => panic!("arbitrary_ext supports only named fields"),
        },
        _ => panic!("arbitrary_ext only supports non-tuple structs"),
    };

    quote!(#inner_impl)
}

fn impl_arbitrary_for_struct(
    struct_name: &Ident,
    fields: &Punctuated<Field, Comma>,
) -> TokenStream2 {
    let field_assigns = fields
        .iter()
        .map(|field| gen_field_assign(field));

    quote! {
        impl<'a> ::arbitrary::Arbitrary<'a> for #struct_name {
            fn arbitrary(u: &mut ::arbitrary::Unstructured<'a>) -> ::arbitrary::Result<Self> {
                let value = Self {
                    #(#field_assigns,)*
                };
                Ok(value)
            }
        }
    }
}

fn gen_field_assign(field: &Field) -> TokenStream2 {
    let field_name = &field.ident;
    let value_source = determine_value_source(field);

    let value = match value_source {
        ValueSource::Default => {
            quote!(
                Default::default()
            )
        }
        ValueSource::Arbitrary => {
            quote!(
                ::arbitrary::Arbitrary::arbitrary(u)?
            )
        }
        ValueSource::CustomFunction(func_path) => {
            quote!(
                #func_path(u)?
            )
        }
    };

    quote!(
        #field_name: #value
    )
}


fn determine_value_source(field: &Field) -> ValueSource {
    let opt_attr = fetch_arbitrary_ext_attr_from_field(field);
    match opt_attr {
        Some(attr) => parse_attribute(attr),
        None => ValueSource::Arbitrary,
    }
}

fn fetch_arbitrary_ext_attr_from_field(field: &Field) -> Option<&Attribute> {
    field.attrs.iter().find(|a| {
        let path = &a.path;
        let name = quote!(#path).to_string();
        name == "arbitrary_ext"
    })
}

fn parse_attribute(attr: &Attribute) -> ValueSource {
    let group = {
        let mut tokens_iter = attr.clone().tokens.into_iter();
        let token = tokens_iter.next().expect("arbitrary_ext attribute cannot be empty");
        match token {
            TokenTree::Group(g) => g,
            t => panic!("Expected group, got: {t:?})")
        }
    };

    let mut tokens_iter = group.stream().into_iter();

    let token = tokens_iter.next().expect("arbitrary_ext attribute cannot be empty");

    match token.to_string().as_ref() {
        "default" => ValueSource::Default,
        "custom" => {
            let eq_sign = tokens_iter.next().unwrap();
            assert_eq!(eq_sign.to_string(), "=", "invalid syntax for arbitrary_ext");
            let func_path: TokenStream2 = tokens_iter.collect();
            ValueSource::CustomFunction(func_path)
        }
        _ => panic!("Unknown options for arbitrary_ext: {}", token)
    }
}

enum ValueSource {
    // Assume that Arbitrary is defined for the type of this field and use it
    Arbitrary,

    // Use Default::default()
    Default,

    // Use custom function to generate a value for a field
    CustomFunction(TokenStream2),
}
