# impl_ops [![Build Status]][travis] [![Latest Version]][crates.io]

[Build Status]: https://api.travis-ci.org/brianwp3000/impl_ops.svg?branch=master
[travis]: https://travis-ci.org/brianwp3000/impl_ops
[Latest Version]: https://img.shields.io/crates/v/impl_ops.svg
[crates.io]: https://crates.io/crates/impl_ops

**NOTICE: This crate is stable, but won't be receiving new features. For a more up-to-date version of this crate see [auto_ops](https://github.com/carbotaniuman/auto_ops).**

Macros for easy operator overloading.

[Documentation](https://docs.rs/impl_ops/)

This library makes writing multiple `impl std::ops::<op>` blocks much faster, especially when you want operators defined for both owned and borrowed variants of the inputs.

To use, include `#[macro_use] extern crate impl_ops;` in your crate and `use std::ops;` in your module. Remember that you can only overload operators between one or more types defined in the current crate.
# Examples
```rust
#[macro_use] extern crate impl_ops;
use std::ops;

#[derive(Clone, Debug, PartialEq)]
struct DonkeyKong {
    pub bananas: i32,
}
impl DonkeyKong {
    pub fn new(bananas: i32) -> DonkeyKong {
        DonkeyKong { bananas: bananas }
    }
}

impl_op_ex!(+ |a: &DonkeyKong, b: &DonkeyKong| -> DonkeyKong { DonkeyKong::new(a.bananas + b.bananas) });
impl_op_ex!(+= |a: &mut DonkeyKong, b: &DonkeyKong| { a.bananas += b.bananas });

fn main() {
    assert_eq!(DonkeyKong::new(5), DonkeyKong::new(4) + DonkeyKong::new(1));
    assert_eq!(DonkeyKong::new(5), DonkeyKong::new(4) + &DonkeyKong::new(1));
    assert_eq!(DonkeyKong::new(5), &DonkeyKong::new(4) + DonkeyKong::new(1));
    assert_eq!(DonkeyKong::new(5), &DonkeyKong::new(4) + &DonkeyKong::new(1));

    let mut dk = DonkeyKong::new(4);
    dk += DonkeyKong::new(1);
    dk += &DonkeyKong::new(1);
    assert_eq!(DonkeyKong::new(6), dk);
}
```

# Roadmap
This crate is stable but no longer receiving feature updates. Please see [auto_ops](https://github.com/carbotaniuman/auto_ops) for a more up-to-date fork of this crate.
