// #![feature(trace_macros)]
// trace_macros!(true);

#[macro_use]
extern crate impl_ops;

use std::ops;

mod kong {
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct Donkey {
        bananas: i32,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct Diddy<T> {
        bananas: T,
    }
}

impl_op_closure!(+ |a: &kong::Donkey, b: &kong::Donkey| -> String {format!("{:?} + {:?}", a, b)});
impl_op_closure!(- |a: &kong::Donkey, b: &kong::Donkey| -> String {format!("{:?} - {:?}", a, b)});
impl_op_closure!(* |a: &kong::Donkey, b: &kong::Donkey| -> String {format!("{:?} * {:?}", a, b)});
impl_op_closure!(/ |a: &kong::Donkey, b: &kong::Donkey| -> String {format!("{:?} / {:?}", a, b)});
impl_op_closure!(% |a: &kong::Donkey, b: &kong::Donkey| -> String {format!("{:?} % {:?}", a, b)});

impl_op_closure!(& |a: &kong::Donkey, b: &kong::Donkey| -> String {format!("{:?} & {:?}", a, b)});
impl_op_closure!(| |a: &kong::Donkey, b: &kong::Donkey| -> String {format!("{:?} | {:?}", a, b)});
impl_op_closure!(^ |a: &kong::Donkey, b: &kong::Donkey| -> String {format!("{:?} ^ {:?}", a, b)});

impl_op_closure!(<< |a: &kong::Donkey, b: &kong::Donkey| -> String {format!("{:?} << {:?}", a, b)});
impl_op_closure!(>> |a: &kong::Donkey, b: &kong::Donkey| -> String {format!("{:?} >> {:?}", a, b)});

impl_op_closure_commutative!(+ |a: &kong::Donkey, b: &kong::Diddy<i32>| -> String {format!("{:?} + {:?}", a, b)});
impl_op_closure_commutative!(- |a: &kong::Donkey, b: &kong::Diddy<i32>| -> String {format!("{:?} - {:?}", a, b)});
impl_op_closure_commutative!(* |a: &kong::Donkey, b: &kong::Diddy<i32>| -> String {format!("{:?} * {:?}", a, b)});
impl_op_closure_commutative!(/ |a: &kong::Donkey, b: &kong::Diddy<i32>| -> String {format!("{:?} / {:?}", a, b)});
impl_op_closure_commutative!(% |a: &kong::Donkey, b: &kong::Diddy<i32>| -> String {format!("{:?} % {:?}", a, b)});

impl_op_closure_commutative!(& |a: &kong::Donkey, b: &kong::Diddy<i32>| -> String {format!("{:?} & {:?}", a, b)});
impl_op_closure_commutative!(| |a: &kong::Donkey, b: &kong::Diddy<i32>| -> String {format!("{:?} | {:?}", a, b)});
impl_op_closure_commutative!(^ |a: &kong::Donkey, b: &kong::Diddy<i32>| -> String {format!("{:?} ^ {:?}", a, b)});

impl_op_closure_commutative!(<< |a: &kong::Donkey, b: &kong::Diddy<i32>| -> String {format!("{:?} << {:?}", a, b)});
impl_op_closure_commutative!(>> |a: &kong::Donkey, b: &kong::Diddy<i32>| -> String {format!("{:?} >> {:?}", a, b)});

macro_rules! impl_op_test {
    ($test:ident, ($lhs:expr) $op:tt ($rhs:expr) = ($expected:expr)) => (
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
    ($test:ident, ($lhs:expr) $op:tt ($rhs:expr) = ($expected:expr)) => (
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

impl_op_test!(add, (kong::Donkey::default()) + (kong::Donkey::default()) = (format!("{:?} + {:?}", kong::Donkey::default(), kong::Donkey::default())));
impl_op_test!(sub, (kong::Donkey::default()) - (kong::Donkey::default()) = (format!("{:?} - {:?}", kong::Donkey::default(), kong::Donkey::default())));
impl_op_test!(mul, (kong::Donkey::default()) * (kong::Donkey::default()) = (format!("{:?} * {:?}", kong::Donkey::default(), kong::Donkey::default())));
impl_op_test!(div, (kong::Donkey::default()) / (kong::Donkey::default()) = (format!("{:?} / {:?}", kong::Donkey::default(), kong::Donkey::default())));
impl_op_test!(rem, (kong::Donkey::default()) % (kong::Donkey::default()) = (format!("{:?} % {:?}", kong::Donkey::default(), kong::Donkey::default())));
impl_op_test!(bitand, (kong::Donkey::default()) & (kong::Donkey::default()) = (format!("{:?} & {:?}", kong::Donkey::default(), kong::Donkey::default())));
impl_op_test!(bitor, (kong::Donkey::default()) | (kong::Donkey::default()) = (format!("{:?} | {:?}", kong::Donkey::default(), kong::Donkey::default())));
impl_op_test!(bitxor, (kong::Donkey::default()) ^ (kong::Donkey::default()) = (format!("{:?} ^ {:?}", kong::Donkey::default(), kong::Donkey::default())));
impl_op_test!(shl, (kong::Donkey::default()) << (kong::Donkey::default()) = (format!("{:?} << {:?}", kong::Donkey::default(), kong::Donkey::default())));
impl_op_test!(shr, (kong::Donkey::default()) >> (kong::Donkey::default()) = (format!("{:?} >> {:?}", kong::Donkey::default(), kong::Donkey::default())));

impl_op_test_commutative!(add_commutative, (kong::Donkey::default()) + (kong::Diddy::<i32>::default()) = (format!("{:?} + {:?}", kong::Donkey::default(), kong::Diddy::<i32>::default())));
impl_op_test_commutative!(sub_commutative, (kong::Donkey::default()) - (kong::Diddy::<i32>::default()) = (format!("{:?} - {:?}", kong::Donkey::default(), kong::Diddy::<i32>::default())));
impl_op_test_commutative!(mul_commutative, (kong::Donkey::default()) * (kong::Diddy::<i32>::default()) = (format!("{:?} * {:?}", kong::Donkey::default(), kong::Diddy::<i32>::default())));
impl_op_test_commutative!(div_commutative, (kong::Donkey::default()) / (kong::Diddy::<i32>::default()) = (format!("{:?} / {:?}", kong::Donkey::default(), kong::Diddy::<i32>::default())));
impl_op_test_commutative!(rem_commutative, (kong::Donkey::default()) % (kong::Diddy::<i32>::default()) = (format!("{:?} % {:?}", kong::Donkey::default(), kong::Diddy::<i32>::default())));
impl_op_test_commutative!(bitand_commutative, (kong::Donkey::default()) & (kong::Diddy::<i32>::default()) = (format!("{:?} & {:?}", kong::Donkey::default(), kong::Diddy::<i32>::default())));
impl_op_test_commutative!(bitor_commutative, (kong::Donkey::default()) | (kong::Diddy::<i32>::default()) = (format!("{:?} | {:?}", kong::Donkey::default(), kong::Diddy::<i32>::default())));
impl_op_test_commutative!(bitxor_commutative, (kong::Donkey::default()) ^ (kong::Diddy::<i32>::default()) = (format!("{:?} ^ {:?}", kong::Donkey::default(), kong::Diddy::<i32>::default())));
impl_op_test_commutative!(shl_commutative, (kong::Donkey::default()) << (kong::Diddy::<i32>::default()) = (format!("{:?} << {:?}", kong::Donkey::default(), kong::Diddy::<i32>::default())));
impl_op_test_commutative!(shr_commutative, (kong::Donkey::default()) >> (kong::Diddy::<i32>::default()) = (format!("{:?} >> {:?}", kong::Donkey::default(), kong::Diddy::<i32>::default())));



