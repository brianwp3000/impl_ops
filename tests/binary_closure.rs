// #![feature(trace_macros)]
// trace_macros!(true);

#[macro_use]
extern crate impl_ops;

use std::ops;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl_op_closure!(+ |a: &Point, b: &Point| -> String {format!("{:?} + {:?}", a, b)});
impl_op_closure!(- |a: &Point, b: &Point| -> String {format!("{:?} - {:?}", a, b)});
impl_op_closure!(* |a: &Point, b: &Point| -> String {format!("{:?} * {:?}", a, b)});
impl_op_closure!(/ |a: &Point, b: &Point| -> String {format!("{:?} / {:?}", a, b)});

impl_op_closure!(& |a: &Point, b: &Point| -> String {format!("{:?} & {:?}", a, b)});
impl_op_closure!(| |a: &Point, b: &Point| -> String {format!("{:?} | {:?}", a, b)});
impl_op_closure!(^ |a: &Point, b: &Point| -> String {format!("{:?} ^ {:?}", a, b)});

impl_op_closure!(% |a: &Point, b: &Point| -> String {format!("{:?} % {:?}", a, b)});

impl_op_closure!(<< |a: &Point, b: &Point| -> String {format!("{:?} << {:?}", a, b)});
impl_op_closure!(>> |a: &Point, b: &Point| -> String {format!("{:?} >> {:?}", a, b)});

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

impl_op_test!(add, (Point {x: 1, y: 2}) + (Point {x: 3, y: 4}) = (format!("{:?} + {:?}", Point {x: 1, y: 2}, Point {x: 3, y: 4})));
impl_op_test!(sub, (Point {x: 1, y: 2}) - (Point {x: 3, y: 4}) = (format!("{:?} - {:?}", Point {x: 1, y: 2}, Point {x: 3, y: 4})));
impl_op_test!(mul, (Point {x: 1, y: 2}) * (Point {x: 3, y: 4}) = (format!("{:?} * {:?}", Point {x: 1, y: 2}, Point {x: 3, y: 4})));
impl_op_test!(div, (Point {x: 1, y: 2}) / (Point {x: 3, y: 4}) = (format!("{:?} / {:?}", Point {x: 1, y: 2}, Point {x: 3, y: 4})));
impl_op_test!(bitand, (Point {x: 1, y: 2}) & (Point {x: 3, y: 4}) = (format!("{:?} & {:?}", Point {x: 1, y: 2}, Point {x: 3, y: 4})));
impl_op_test!(bitor, (Point {x: 1, y: 2}) | (Point {x: 3, y: 4}) = (format!("{:?} | {:?}", Point {x: 1, y: 2}, Point {x: 3, y: 4})));
impl_op_test!(bitxor, (Point {x: 1, y: 2}) ^ (Point {x: 3, y: 4}) = (format!("{:?} ^ {:?}", Point {x: 1, y: 2}, Point {x: 3, y: 4})));
impl_op_test!(rem, (Point {x: 1, y: 2}) % (Point {x: 3, y: 4}) = (format!("{:?} % {:?}", Point {x: 1, y: 2}, Point {x: 3, y: 4})));
impl_op_test!(shl, (Point {x: 1, y: 2}) << (Point {x: 3, y: 4}) = (format!("{:?} << {:?}", Point {x: 1, y: 2}, Point {x: 3, y: 4})));
impl_op_test!(shr, (Point {x: 1, y: 2}) >> (Point {x: 3, y: 4}) = (format!("{:?} >> {:?}", Point {x: 1, y: 2}, Point {x: 3, y: 4})));
