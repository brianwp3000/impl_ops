# impl_ops
Macros for overloading operators easily in Rust.

The ```impl_op!``` macro expands an operator and a closure a trait implementation for the relevant type. If either type is a reference, it creates trait implementations for both owned and borrowed variants of the type.

The ```impl_op_commutative!``` macro can be used with commutative binary operations of two different types. 

## Usage
```rust
#[macro_use]
extern crate impl_ops;

use std::ops;

#[derive(Clone, Debug)]
struct Barrel {
  pub bananas: i32,
}

// Binary operators:
// impl_op!(<OP> |a: <LHS>, b: <RHS>| -> OUT {<BODY>});

impl_op!(+ |a: &Barrel, b: &Barrel| -> Barrel { 
  Barrel { bananas: a.bananas + b.bananas }
});

// Commutative binary operators:
// impl_op!(<OP> |a: <LHS>, b: <RHS>| -> OUT {<BODY>});
// (where LHS != RHS)

impl_op!(+ |a: &Barrel, b: i32| -> Barrel { 
  Barrel { bananas: a.bananas + b }
});

// Assignment operators:
// impl_op!(<OP> |a: &mut <LHS>, b: <RHS>| {<BODY>});

impl_op!(+= |a: &mut Barrel, b: &Barrel| { 
  a.bananas += b.bananas;
});

// Unary operators (-, !):
// impl_op!(<OP> |a: <LHS>| -> <OUT> {<BODY>});

impl_op!(- |a: &Barrel| -> Barrel { 
  Barrel { bananas = -a.bananas }
});

fn main() {
  let b1 = Barrel { bananas: 1 };
  let b2 = Barrel { bananas: 2 };
  
  // binary
  println!("{:?}", b1.clone() + b2.clone());
  println!("{:?}", b1.clone() + &b2);
  println!("{:?}", &b1 + b2.clone());
  println!("{:?}", &b1 + &b2);
  
  // commutative binary
  println!("{:?}", b1.clone() + 1);
  println!("{:?}", 1 + b2.clone());
  
  // assignment
  b1 += b2.clone();
  println!("{:?}", b1);
  b1 += &b2;
  println!("{:?}", b1);
  
  // unary
  println!("{:?}", -b1.clone());
  println!("{:?}", -&b1);
}
```

## Limitations
- Index, IndexMut, Deref, and DerefMut haven't been implemented yet
- Generics aren't supported, i.e.
```rust
impl_op!(+ |a: &Barrel<T>, b &Barrel<T>| -> Barrel<T> {...}); // BAD
impl_op!(+ |a: &Barrel<i32>, b &Barrel<i32>| -> Barrel<i32> {...}); // GOOD
```
