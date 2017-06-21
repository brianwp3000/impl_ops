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

    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct Diddy<T> {
        pub bananas: T,
    }
}

impl_op!(+ |a: &kong::Donkey, b: &kong::Donkey| -> String {
    let total_bananas = a.bananas + b.bananas;
    format!("{:?} + {:?} -> {:?}", a, b, total_bananas)
});
impl_op!(- |a: &kong::Donkey, b: &kong::Donkey| -> String {format!("{:?} - {:?}", a, b)});
impl_op!(* |a: &kong::Donkey, b: &kong::Donkey| -> String {format!("{:?} * {:?}", a, b)});
impl_op!(/ |a: &kong::Donkey, b: &kong::Donkey| -> String {format!("{:?} / {:?}", a, b)});
impl_op!(% |a: &kong::Donkey, b: &kong::Donkey| -> String {format!("{:?} % {:?}", a, b)});

impl_op!(& |a: &kong::Donkey, b: &kong::Donkey| -> String {format!("{:?} & {:?}", a, b)});
impl_op!(| |a: &kong::Donkey, b: &kong::Donkey| -> String {format!("{:?} | {:?}", a, b)});
impl_op!(^ |a: &kong::Donkey, b: &kong::Donkey| -> String {format!("{:?} ^ {:?}", a, b)});

impl_op!(<< |a: &kong::Donkey, b: &kong::Donkey| -> String {format!("{:?} << {:?}", a, b)});
impl_op!(>> |a: &kong::Donkey, b: &kong::Donkey| -> String {format!("{:?} >> {:?}", a, b)});

impl_op!(+ |a: &kong::Diddy<i32>, b: &kong::Diddy<i32>| -> String {format!("{:?} + {:?}", a, b)});
impl_op!(- |a: &kong::Diddy<i32>, b: kong::Diddy<i32>| -> String {format!("{:?} - {:?}", a, b)});
impl_op!(* |a: kong::Diddy<i32>, b: &kong::Diddy<i32>| -> String {format!("{:?} * {:?}", a, b)});
impl_op!(/ |a: kong::Diddy<i32>, b: kong::Diddy<i32>| -> String {format!("{:?} / {:?}", a, b)});

impl_op_commutative!(+ |a: &kong::Diddy<i32>, b: &i32| -> String {format!("{:?} + {:?}", a, b)});
impl_op_commutative!(- |a: &kong::Diddy<i32>, b: i32| -> String {format!("{:?} - {:?}", a, b)});
impl_op_commutative!(* |a: kong::Diddy<i32>, b: &i32| -> String {format!("{:?} * {:?}", a, b)});
impl_op_commutative!(/ |a: kong::Diddy<i32>, b: i32| -> String {format!("{:?} / {:?}", a, b)});

impl_op_commutative!(+ |a: &kong::Donkey, b: &kong::Diddy<i32>| -> String {
    let total_bananas = a.bananas + b.bananas;
    format!("{:?} + {:?} -> {:?}", a, b, total_bananas)
});
impl_op_commutative!(- |a: &kong::Donkey, b: &kong::Diddy<i32>| -> String {format!("{:?} - {:?}", a, b)});
impl_op_commutative!(* |a: &kong::Donkey, b: &kong::Diddy<i32>| -> String {format!("{:?} * {:?}", a, b)});
impl_op_commutative!(/ |a: &kong::Donkey, b: &kong::Diddy<i32>| -> String {format!("{:?} / {:?}", a, b)});
impl_op_commutative!(% |a: &kong::Donkey, b: &kong::Diddy<i32>| -> String {format!("{:?} % {:?}", a, b)});

impl_op_commutative!(& |a: &kong::Donkey, b: &kong::Diddy<i32>| -> String {format!("{:?} & {:?}", a, b)});
impl_op_commutative!(| |a: &kong::Donkey, b: &kong::Diddy<i32>| -> String {format!("{:?} | {:?}", a, b)});
impl_op_commutative!(^ |a: &kong::Donkey, b: &kong::Diddy<i32>| -> String {format!("{:?} ^ {:?}", a, b)});

impl_op_commutative!(<< |a: &kong::Donkey, b: &kong::Diddy<i32>| -> String {format!("{:?} << {:?}", a, b)});
impl_op_commutative!(>> |a: &kong::Donkey, b: &kong::Diddy<i32>| -> String {format!("{:?} >> {:?}", a, b)});

macro_rules! impl_op_test {
    ($test:ident, ($lhs:expr) $op:tt ($rhs:expr) -> ($expected:expr)) => (
        #[test]
        fn $test() {
            let lhs = $lhs;
            let rhs = $rhs;
            let expected = $expected;

            let actual = lhs $op rhs;
            assert_eq!(expected, actual, "owned <op> owned");
            let actual = lhs $op &rhs;
            assert_eq!(expected, actual, "owned <op> borrowed");
            let actual = &lhs $op rhs;
            assert_eq!(expected, actual, "borrowed <op> owned");
            let actual = &lhs $op &rhs;
            assert_eq!(expected, actual, "borrowed <op> borrowed");
        }
    );
}

macro_rules! impl_op_test_commutative {
    ($test:ident, ($lhs:expr) $op:tt ($rhs:expr) -> ($expected:expr)) => (
        #[test]
        fn $test() {
            let lhs = $lhs;
            let rhs = $rhs;
            let expected = $expected;

            let actual = lhs $op rhs;
            assert_eq!(expected, actual, "owned <op> owned");
            let actual = lhs $op &rhs;
            assert_eq!(expected, actual, "owned <op> borrowed");
            let actual = &lhs $op rhs;
            assert_eq!(expected, actual, "borrowed <op> owned");
            let actual = &lhs $op &rhs;
            assert_eq!(expected, actual, "borrowed <op> borrowed");

            let actual = rhs $op lhs;
            assert_eq!(expected, actual, "owned <op> owned [commutative]");
            let actual = rhs $op lhs;
            assert_eq!(expected, actual, "owned <op> borrowed [commutative]");
            let actual = &rhs $op lhs;
            assert_eq!(expected, actual, "borrowed <op> owned [commutative]");
            let actual = &rhs $op &lhs;
            assert_eq!(expected, actual, "borrowed <op> borrowed [commutative]");
        }
    );
}

impl_op_test!(add, (kong::Donkey::default()) + (kong::Donkey::default()) -> (format!("{:?} + {:?} -> {:?}", kong::Donkey::default(), kong::Donkey::default(), 0)));
impl_op_test!(sub, (kong::Donkey::default()) - (kong::Donkey::default()) -> (format!("{:?} - {:?}", kong::Donkey::default(), kong::Donkey::default())));
impl_op_test!(mul, (kong::Donkey::default()) * (kong::Donkey::default()) -> (format!("{:?} * {:?}", kong::Donkey::default(), kong::Donkey::default())));
impl_op_test!(div, (kong::Donkey::default()) / (kong::Donkey::default()) -> (format!("{:?} / {:?}", kong::Donkey::default(), kong::Donkey::default())));
impl_op_test!(rem, (kong::Donkey::default()) % (kong::Donkey::default()) -> (format!("{:?} % {:?}", kong::Donkey::default(), kong::Donkey::default())));
impl_op_test!(bitand, (kong::Donkey::default()) & (kong::Donkey::default()) -> (format!("{:?} & {:?}", kong::Donkey::default(), kong::Donkey::default())));
impl_op_test!(bitor, (kong::Donkey::default()) | (kong::Donkey::default()) -> (format!("{:?} | {:?}", kong::Donkey::default(), kong::Donkey::default())));
impl_op_test!(bitxor, (kong::Donkey::default()) ^ (kong::Donkey::default()) -> (format!("{:?} ^ {:?}", kong::Donkey::default(), kong::Donkey::default())));
impl_op_test!(shl, (kong::Donkey::default()) << (kong::Donkey::default()) -> (format!("{:?} << {:?}", kong::Donkey::default(), kong::Donkey::default())));
impl_op_test!(shr, (kong::Donkey::default()) >> (kong::Donkey::default()) -> (format!("{:?} >> {:?}", kong::Donkey::default(), kong::Donkey::default())));

impl_op_test_commutative!(add_commutative, (kong::Donkey::default()) + (kong::Diddy::<i32>::default()) -> (format!("{:?} + {:?} -> {:?}", kong::Donkey::default(), kong::Diddy::<i32>::default(), 0)));
impl_op_test_commutative!(sub_commutative, (kong::Donkey::default()) - (kong::Diddy::<i32>::default()) -> (format!("{:?} - {:?}", kong::Donkey::default(), kong::Diddy::<i32>::default())));
impl_op_test_commutative!(mul_commutative, (kong::Donkey::default()) * (kong::Diddy::<i32>::default()) -> (format!("{:?} * {:?}", kong::Donkey::default(), kong::Diddy::<i32>::default())));
impl_op_test_commutative!(div_commutative, (kong::Donkey::default()) / (kong::Diddy::<i32>::default()) -> (format!("{:?} / {:?}", kong::Donkey::default(), kong::Diddy::<i32>::default())));
impl_op_test_commutative!(rem_commutative, (kong::Donkey::default()) % (kong::Diddy::<i32>::default()) -> (format!("{:?} % {:?}", kong::Donkey::default(), kong::Diddy::<i32>::default())));
impl_op_test_commutative!(bitand_commutative, (kong::Donkey::default()) & (kong::Diddy::<i32>::default()) -> (format!("{:?} & {:?}", kong::Donkey::default(), kong::Diddy::<i32>::default())));
impl_op_test_commutative!(bitor_commutative, (kong::Donkey::default()) | (kong::Diddy::<i32>::default()) -> (format!("{:?} | {:?}", kong::Donkey::default(), kong::Diddy::<i32>::default())));
impl_op_test_commutative!(bitxor_commutative, (kong::Donkey::default()) ^ (kong::Diddy::<i32>::default()) -> (format!("{:?} ^ {:?}", kong::Donkey::default(), kong::Diddy::<i32>::default())));
impl_op_test_commutative!(shl_commutative, (kong::Donkey::default()) << (kong::Diddy::<i32>::default()) -> (format!("{:?} << {:?}", kong::Donkey::default(), kong::Diddy::<i32>::default())));
impl_op_test_commutative!(shr_commutative, (kong::Donkey::default()) >> (kong::Diddy::<i32>::default()) -> (format!("{:?} >> {:?}", kong::Donkey::default(), kong::Diddy::<i32>::default())));

#[test]
fn binary_borrowed_borrowed() {
    let lhs = kong::Diddy::<i32>::default();
    let rhs = kong::Diddy::<i32>::default();
    let expected = format!("{:?} + {:?}", lhs, rhs);

    let actual = lhs + rhs;
    assert_eq!(expected, actual, "owned <op> owned");
    let actual = lhs + &rhs;
    assert_eq!(expected, actual, "owned <op> borrowed");
    let actual = &lhs + rhs;
    assert_eq!(expected, actual, "borrowed <op> owned");
    let actual = &lhs + &rhs;
    assert_eq!(expected, actual, "borrowed <op> borrowed");
}

#[test]
fn binary_borrowed_owned() {
    let lhs = kong::Diddy::<i32>::default();
    let rhs = kong::Diddy::<i32>::default();
    let expected = format!("{:?} - {:?}", lhs, rhs);

    let actual = lhs - rhs;
    assert_eq!(expected, actual, "owned <op> owned");
    let actual = &lhs - rhs;
    assert_eq!(expected, actual, "borrowed <op> owned");
}

#[test]
fn binary_owned_borrowed() {
    let lhs = kong::Diddy::<i32>::default();
    let rhs = kong::Diddy::<i32>::default();
    let expected = format!("{:?} * {:?}", lhs, rhs);

    let actual = lhs * rhs;
    assert_eq!(expected, actual, "owned <op> owned");
    let actual = lhs * &rhs;
    assert_eq!(expected, actual, "owned <op> borrowed");
}

#[test]
fn binary_owned_owned() {
    let lhs = kong::Diddy::<i32>::default();
    let rhs = kong::Diddy::<i32>::default();
    let expected = format!("{:?} / {:?}", lhs, rhs);

    let actual = lhs / rhs;
    assert_eq!(expected, actual, "owned <op> owned");
}

#[test]
fn binary_borrowed_borrowed_commutative() {
    let lhs = kong::Diddy::<i32>::default();
    let rhs = 1;
    let expected = format!("{:?} + {:?}", lhs, rhs);

    let actual = lhs + rhs;
    assert_eq!(expected, actual, "owned <op> owned");
    let actual = lhs + &rhs;
    assert_eq!(expected, actual, "owned <op> borrowed");
    let actual = &lhs + rhs;
    assert_eq!(expected, actual, "borrowed <op> owned");
    let actual = &lhs + &rhs;
    assert_eq!(expected, actual, "borrowed <op> borrowed");

    let actual = rhs + lhs;
    assert_eq!(expected, actual, "owned <op> owned [commutative]");
    let actual = rhs + &lhs;
    assert_eq!(expected, actual, "owned <op> borrowed [commutative]");
    let actual = &rhs + lhs;
    assert_eq!(expected, actual, "borrowed <op> owned [commutative]");
    let actual = &rhs + &lhs;
    assert_eq!(expected, actual, "borrowed <op> borrowed [commutative]");
}

#[test]
fn binary_borrowed_owned_commutative() {
    let lhs = kong::Diddy::<i32>::default();
    let rhs = 1i32;
    let expected = format!("{:?} - {:?}", lhs, rhs);

    let actual = lhs - rhs;
    assert_eq!(expected, actual, "owned <op> owned");
    let actual = &lhs - rhs;
    assert_eq!(expected, actual, "borrowed <op> owned");

    let actual = rhs - lhs;
    assert_eq!(expected, actual, "owned <op> owned [commutative]");
    let actual = rhs - &lhs;
    assert_eq!(expected, actual, "owned <op> borrowed [commutative]");
}

#[test]
fn binary_owned_borrowed_commutative() {
    let lhs = kong::Diddy::<i32>::default();
    let rhs = 1;
    let expected = format!("{:?} * {:?}", lhs, rhs);

    let actual = lhs * rhs;
    assert_eq!(expected, actual, "owned <op> owned");
    let actual = lhs * &rhs;
    assert_eq!(expected, actual, "owned <op> borrowed");

    let actual = rhs * lhs;
    assert_eq!(expected, actual, "owned <op> owned [commutative]");
    let actual = &rhs * lhs;
    assert_eq!(expected, actual, "owned <op> borrowed [commutative]");
}

#[test]
fn binary_owned_owned_commutative() {
    let lhs = kong::Diddy::<i32>::default();
    let rhs = 1;
    let expected = format!("{:?} / {:?}", lhs, rhs);

    let actual = lhs / rhs;
    assert_eq!(expected, actual, "owned <op> owned");

    let actual = rhs / lhs;
    assert_eq!(expected, actual, "owned <op> owned [commutative]");
}


