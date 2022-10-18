# Arbitrary Ext

`ArbitraryExt` is an extension of [Arbitrary](https://github.com/rust-fuzz/arbitrary) crate that provides a way
to derive `Arbitrary` trait but set custom implementation for single fields.

## Usage

```rust
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
```

Output:

```
Point { x: 84, y: 0, z: 1869294 }
```

## Note

There is an [Arbitrary issue](https://github.com/rust-fuzz/arbitrary/issues/33) that requires exactly this, but was not yet approached in 2 years.
This crate exists just as a workaround.
