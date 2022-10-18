use arbitrary::{Arbitrary, Unstructured};
use arbitrary_ext::ArbitraryExt;

#[derive(Debug, ArbitraryExt)]
struct Point {
    #[arbitrary_ext(custom = arbitrary_x)]
    x: i32,

    #[arbitrary_ext(default)]
    y: i32,

    z: i32,
}

fn arbitrary_x(u: &mut Unstructured) -> arbitrary::Result<i32> {
    u.int_in_range(0..=100)
}

fn main() {
    let mut u = Unstructured::new(&[0x54, 0xee, 0x85, 0x1c]);
    let point = Point::arbitrary(&mut u).unwrap();
    println!("{point:?}");
}
