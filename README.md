# impl_ops [![Build Status]][travis]

[Build Status]: https://api.travis-ci.org/brianwp3000/impl_ops.svg?branch=master
[travis]: https://travis-ci.org/brianwp3000/impl_ops

Macros for easy operator overloading.

[Documentation](https://docs.rs/impl_ops/)

This library make writing multiple `impl std::ops::<op>` blocks much faster, especially when you want operators defined for both owned and borrowed variants of the inputs.

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
The syntax is essentially finished, but I still need to implement `Index`, `IndexMut`, `Deref`, and `DerefMut`. I'm already using this in a few personal projects, but I want to make sure it works in more real-world scenarios before committing to maintain a stable API. 
