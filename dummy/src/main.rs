use arbitrary::{Arbitrary, Unstructured};
use arbitrary_ext::ArbitraryExt;

/*
#[derive(Debug, ArbitraryExt)]
struct Point {
    #[arbitrary_ext(with = "arbitrary_x")]
    x: i32,

    #[arbitrary_ext(default)]
    y: i32,

    #[arbitrary_ext(value = "100 + 50")]
    z: i32,

    a: i32,
}

#[derive(Debug, ArbitraryExt)]
struct Age(#[arbitrary_ext(with = "arbitrary_x")] i32);

#[derive(Debug, ArbitraryExt)]
enum FooBar {
    Foo(i32),
    Bar {
        #[arbitrary_ext(with = "arbitrary_x")]
        x: i32,

        y: i32,
    },
}

fn arbitrary_x(u: &mut Unstructured) -> arbitrary::Result<i32> {
    u.int_in_range(0..=100)
}
*/

#[derive(Debug, ArbitraryExt)]
struct User {
    #[arbitrary_ext(with = "arbitrary_username")]
    user_name: UserName,

    age: u16,
}

#[derive(Debug)]
struct UserName(String);

fn arbitrary_username(u: &mut Unstructured) -> arbitrary::Result<UserName> {
    let name = String::arbitrary(u)?;
    Ok(UserName(name))
}

fn main() {
    use rand::RngCore;
    let mut bytes = [0u8; 2048];
    rand::thread_rng().fill_bytes(&mut bytes);
    let mut u = Unstructured::new(&bytes);

    /*
    let point = Point::arbitrary(&mut u).unwrap();
    println!("{point:?}");

    let age = Age::arbitrary(&mut u).unwrap();
    println!("{age:?}");

    let foobar = FooBar::arbitrary(&mut u).unwrap();
    println!("{foobar:?}");
    */

    let user = User::arbitrary(&mut u).unwrap();
    println!("{user:?}");
}
