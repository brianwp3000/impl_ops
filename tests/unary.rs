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

impl_op!(- |a: &kong::Donkey| -> kong::Diddy<i32> {
    let lhs = a;
    kong::Diddy::<i32>::new(lhs.bananas)
});
impl_op!(! |a: &kong::Donkey| -> kong::Diddy<i32> { kong::Diddy::<i32>::new(a.bananas)});

impl_op!(- |a: kong::Diddy<i32>| -> kong::Donkey {
    let lhs = a;
    kong::Donkey::new(lhs.bananas)
});

macro_rules! impl_op_test {
    ($test:ident, $op:tt($lhs:expr) -> ($expected:expr)) => (
        #[test]
        fn $test() {
            let lhs = $lhs;
            let expected = $expected;

            let actual = $op lhs;
            assert_eq!(expected, actual, "<op> owned");
            let actual = $op &lhs;
            assert_eq!(expected, actual, "<op> borrowed");
        }
    );
}

impl_op_test!(neg, -(kong::Donkey::new(2)) -> (kong::Diddy::<i32>::new(2)));
impl_op_test!(not, !(kong::Donkey::new(2)) -> (kong::Diddy::<i32>::new(2)));

#[test]
fn owned() {
    let lhs = kong::Diddy::<i32>::new(2);
    let expected = kong::Donkey::new(2);

    let actual = -lhs;
    assert_eq!(expected, actual, "<op> owned");
}
