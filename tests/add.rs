#![feature(trace_macros)]

#[macro_use]
extern crate impl_ops;

use std::ops;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl impl_ops::Add for Point {
    type Output = Point;
    fn add(lhs: &Point, rhs: &Point) -> Self::Output {
        Point {
            x: lhs.x + rhs.x,
            y: lhs.y + rhs.y,
        }
    }
}

impl impl_ops::Add<i32> for Point {
    type Output = String;
    fn add(lhs: &Point, rhs: &i32) -> Self::Output {
        format!("{:?} {:?}", lhs, rhs)
    }
}

impl impl_ops::Add<f32> for Point {
    type Output = String;
    fn add(lhs: &Point, rhs: &f32) -> Self::Output {
        format!("{:?} {:?}", lhs, rhs)
    }
}

fn add_vec_point(lhs: &std::vec::Vec<i32>, rhs: &Point) -> String {
    format!("{:?} {:?}", lhs, rhs)
}

impl_op!((Point) + (Point) = (Point));
impl_op!((Point) + (i32) = (String));
impl_op!((std::vec::Vec<i32>) + (Point) = (String), add_vec_point);

#[test]
fn add() {
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
fn add_rhs_out() {
    let lhs = Point {x: 1, y: 2};
    let rhs = 3;
    let expected = format!("{:?} {:?}", lhs, rhs);

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
fn vec_i32_str() {
    let lhs = vec!(1, 2, 3);
    let rhs = Point { x: 1, y: 2 };
    let expected = format!("{:?} {:?}", lhs, rhs);

    let actual = lhs.clone() + rhs.clone();
    assert_eq!(expected, actual, "owned <op> owned");
    let actual = lhs.clone() + &rhs;
    assert_eq!(expected, actual, "owned <op> borrowed");
    let actual = &lhs + rhs.clone();
    assert_eq!(expected, actual, "borrowed <op> owned");
    let actual = &lhs + &rhs;
    assert_eq!(expected, actual, "borrowed <op> borrowed");
}

#[test]
fn no_std_child_still_works_in_parent_mod() {
    let lhs = std_mod::OtherStruct {field: 1};
    let rhs = Point {x: 3, y: 4};
    let expected = format!("{:?} {:?}", lhs, rhs);

    let actual = lhs + rhs;
    assert_eq!(expected, actual, "owned <op> owned");
}

mod std_mod {
    use std::ops;
    use impl_ops;

    #[derive(Clone, Debug, PartialEq)]
    pub struct OtherStruct {
        pub field: i32,
    }

    impl impl_ops::Add<super::Point> for OtherStruct {
        type Output = String;
        fn add(lhs: &Self, rhs: &super::Point) -> Self::Output {
            format!("{:?} {:?}", lhs, rhs)
        }
    }

    impl_op!((OtherStruct) + (super::Point) = (String));

    #[test]
    fn parent_still_works_in_std_mod() {
        let lhs = super::Point {x: 1, y: 2};
        let rhs = super::Point {x: 3, y: 4};
        let expected = super::Point {x:4, y: 6};

        let actual = lhs + rhs;
        assert_eq!(expected, actual, "owned <op> owned");
    }
}

mod no_std_mod {
    use super::Point;

    #[test]
    fn parent_still_works_in_no_std_mod() {
        let lhs = Point {x: 1, y: 2};
        let rhs = Point {x: 3, y: 4};
        let expected = Point {x:4, y: 6};

        let actual = lhs + rhs;
        assert_eq!(expected, actual, "owned <op> owned");
    }

    #[test]
    fn sibling_still_works_in_no_std_mod() {
        let lhs = super::std_mod::OtherStruct {field: 1};
        let rhs = Point {x: 3, y: 4};
        let expected = format!("{:?} {:?}", lhs, rhs);

        let actual = lhs + rhs;
        assert_eq!(expected, actual, "owned <op> owned");
    }
}
