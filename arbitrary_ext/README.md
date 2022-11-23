# Arbitrary Ext

Since `1.2.0` Arbitrary supports custom arbitrary implementation for fields on derive.

But it still remains tricky, if a type that does not implement `Arbitrary` is wrapped into a generic type.

This crate provides s set of function combinators to support the containers and collections form the standard library.

See the example below.


## Usage

```rust
use arbitrary_ext::{arbitrary_option, arbitrary_vec, arbitrary_hash_map};
use std::collections::HashMap;

// Imagine this is a foreign type, that by some reason does not implement Arbitrary trait.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Number(u32);

// Our custom function to generate arbitrary Number out of Unstructured.
fn arbitrary_number(u: &mut Unstructured) -> arbitrary::Result<Number> {
    let value = u.int_in_range(0..=1000)?;
    Ok(Number(value))
}

#[derive(Debug, Arbitrary)]
struct Example {
    #[arbitrary(with = arbitrary_number)]
    number: Number,

    #[arbitrary(with = arbitrary_option(arbitrary_number))]
    option: Option<Number>,

    #[arbitrary(with = arbitrary_vec(arbitrary_number))]
    vec: Vec<Number>,

    #[arbitrary(
        with = arbitrary_hash_map(
            arbitrary_number,
            arbitrary_vec(arbitrary_option(arbitrary_number))
        )
    )]
    hash_map: HashMap<Number, Vec<Option<Number>>>,
}
```

Without having `arbitrary_option`, `arbitrary_vec`, `arbitrary_hash_map` combinators, would be forced to implement out custom functions to generate
arbitrary `Option<Number>`, `Vec<Number>` and `HashMap<Number, Vec<Option<Number>>>`. e.g.:

```rust
fn arbitrary_option_number(u: &mut Unstructured) -> arbitrary::Result<Option<Number>>;
fn arbitrary_vec_number(u: &mut Unstructured) -> arbitrary::Result<Vec<Number>>;
fn arbitrary_hash_map_of_numbers(u: &mut Unstructured) -> arbitrary::Result<HashMap<Number, Vec<Option<Number>>>>;
```

But this becomes tedious very quickly.

## History of the crate

Initially the crate was created to workaround
this [Arbitrary issue](https://github.com/rust-fuzz/arbitrary/issues/33) but it was later addressed in
[this PR](https://github.com/rust-fuzz/arbitrary/pull/129).
