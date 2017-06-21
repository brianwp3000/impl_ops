// #![feature(trace_macros)]
// trace_macros!(true);

#[macro_use]
extern crate impl_ops;

use std::ops;

mod kong {
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct Donkey {
        pub bananas: i32,
    }

    impl Donkey {
        pub fn new(bananas: i32) -> Donkey {
            Donkey { bananas }
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct Diddy<T> {
        pub bananas: T,
    }

    impl Diddy<i32> {
        pub fn new(bananas: i32) -> Diddy<i32> {
            Diddy { bananas }
        }
    }
}

impl_op!(+= |a: &mut kong::Donkey, b: &kong::Diddy<i32>| {
    let lhs = a;
    let rhs = b;
    lhs.bananas += rhs.bananas;
});
impl_op!(-= |a: &mut kong::Donkey, b: &kong::Diddy<i32>| {a.bananas += b.bananas;});
impl_op!(*= |a: &mut kong::Donkey, b: &kong::Diddy<i32>| {a.bananas += b.bananas;});
impl_op!(/= |a: &mut kong::Donkey, b: &kong::Diddy<i32>| {a.bananas += b.bananas;});
impl_op!(%= |a: &mut kong::Donkey, b: &kong::Diddy<i32>| {a.bananas += b.bananas;});

impl_op!(&= |a: &mut kong::Donkey, b: &kong::Diddy<i32>| {a.bananas += b.bananas;});
impl_op!(|= |a: &mut kong::Donkey, b: &kong::Diddy<i32>| {a.bananas += b.bananas;});
impl_op!(^= |a: &mut kong::Donkey, b: &kong::Diddy<i32>| {a.bananas += b.bananas;});

impl_op!(<<= |a: &mut kong::Donkey, b: &kong::Diddy<i32>| {a.bananas += b.bananas;});
impl_op!(>>= |a: &mut kong::Donkey, b: &kong::Diddy<i32>| {a.bananas += b.bananas;});

impl_op!(+= |a: &mut kong::Diddy<i32>, b: kong::Donkey| {
    let lhs = a;
    let rhs = b;
    lhs.bananas += rhs.bananas;
});

macro_rules! impl_op_test {
    ($test:ident, ($lhs:expr) $op:tt ($rhs:expr) -> ($expected:expr)) => (
        #[test]
        fn $test() {
            let lhs_orig = $lhs;
            let rhs = $rhs;
            let expected = $expected;

            let mut actual = lhs_orig.clone();
            actual $op rhs;
            assert_eq!(expected, actual, "mut borrowed <op> owned");
            let mut actual = lhs_orig.clone();
            actual $op &rhs;
            assert_eq!(expected, actual, "mut borrowed <op> borrowed");
        }
    );
}

impl_op_test!(add_assign, (kong::Donkey::new(2)) += (kong::Diddy::<i32>::new(1)) -> (kong::Donkey::new(3)));
impl_op_test!(sub_assign, (kong::Donkey::new(2)) -= (kong::Diddy::<i32>::new(1)) -> (kong::Donkey::new(3)));
impl_op_test!(mul_assign, (kong::Donkey::new(2)) *= (kong::Diddy::<i32>::new(1)) -> (kong::Donkey::new(3)));
impl_op_test!(div_assign, (kong::Donkey::new(2)) /= (kong::Diddy::<i32>::new(1)) -> (kong::Donkey::new(3)));
impl_op_test!(rem_assign, (kong::Donkey::new(2)) %= (kong::Diddy::<i32>::new(1)) -> (kong::Donkey::new(3)));
impl_op_test!(bitand_assign, (kong::Donkey::new(2)) &= (kong::Diddy::<i32>::new(1)) -> (kong::Donkey::new(3)));
impl_op_test!(bitor_assign, (kong::Donkey::new(2)) |= (kong::Diddy::<i32>::new(1)) -> (kong::Donkey::new(3)));
impl_op_test!(bitxor_assign, (kong::Donkey::new(2)) ^= (kong::Diddy::<i32>::new(1)) -> (kong::Donkey::new(3)));
impl_op_test!(shl_assign, (kong::Donkey::new(2)) <<= (kong::Diddy::<i32>::new(1)) -> (kong::Donkey::new(3)));
impl_op_test!(shr_assign, (kong::Donkey::new(2)) >>= (kong::Diddy::<i32>::new(1)) -> (kong::Donkey::new(3)));

#[test]
fn owned() {
    let mut actual = kong::Diddy::<i32>::new(2);
    let rhs = kong::Donkey::new(3);
    let expected = kong::Diddy::<i32>::new(5);

    actual += rhs;
    assert_eq!(expected, actual, "mut borrowed <op> owned");
}