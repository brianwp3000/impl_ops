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

fn add_points(lhs: &Point, rhs: &Point) -> Point {
    Point {
        x: lhs.x + rhs.x,
        y: lhs.y + rhs.y,
    }
}

fn add_point_f32(lhs: &Point, rhs: &f32) -> String {
    format!("{:?} + {:?}", lhs, rhs)
}

impl_op!((Point) + (Point) = (Point), add_points);
impl_op!((Point) + (i32) = (String), |a, b| format!("{:?} + {:?}", a, b));

impl_op_commutative!((Point) + (f32) = (String), add_point_f32);
impl_op_commutative!((Point) + (bool) = (String), |a, b| format!("{:?} + {:?}", a, b));

#[test]
fn function() {
    let lhs = Point {x: 1, y: 2};
    let rhs = Point {x: 3, y: 4};
    let expected = Point {x:4, y: 6};

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
fn closure() {
    let lhs = Point {x: 1, y: 2};
    let rhs = 3;
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
fn function_commutative() {
    let lhs = Point {x: 1, y: 2};
    let rhs = 3.0;
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
    assert_eq!(expected, actual, "owned <op> owned");
    let actual = rhs + &lhs;
    assert_eq!(expected, actual, "owned <op> borrowed");
    let actual = &rhs + lhs;
    assert_eq!(expected, actual, "borrowed <op> owned");
    let actual = &rhs + &lhs;
    assert_eq!(expected, actual, "borrowed <op> borrowed");
}

#[test]
fn closure_commutative() {
    let lhs = Point {x: 1, y: 2};
    let rhs = true;
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
    assert_eq!(expected, actual, "owned <op> owned");
    let actual = rhs + &lhs;
    assert_eq!(expected, actual, "owned <op> borrowed");
    let actual = &rhs + lhs;
    assert_eq!(expected, actual, "borrowed <op> owned");
    let actual = &rhs + &lhs;
    assert_eq!(expected, actual, "borrowed <op> borrowed");
}


