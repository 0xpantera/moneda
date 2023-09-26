#![allow(dead_code)]
use std::ops::{Add};
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: Option<isize>,
    y: Option<isize>,
    a: isize,
    b: isize,
}

impl Point {
    fn from(x: Option<isize>, y: Option<isize>, a: isize, b: isize) -> Self {
        if let (Some(xs), Some(ys)) = (x, y) {
            if ys^2 != xs^3 + a * xs + b {
                panic!("({}, {}) is not on the curve", xs, ys);
            } else {
                return Point {x, y, a, b};
            }
        }

        Self { x: None, y: None, a, b, }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && 
        self.y == other.y &&
        self.a == other.a &&
        self.b == other.b
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point({:?},{:?})_{}_{}", self.x, self.y, self.a, self.b)
    }
}


impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        if self.a != rhs.a || self.b != rhs.b {
            panic!("Points {}, {} are not on the same curve", self, rhs);
        }

        // one of the points is the point at infinit
        // P + 0 = P or 0 + P = P
        if self.x.is_none() { return rhs; }
        if rhs.x.is_none() { return self; }

        // the x coordinate of both points is the same
        // the line is vertical and doesn't intersect at any other point
        // so the result is the point at infinity O
        if self.x == rhs.x && self.y != rhs.y {
            return Self { x: None, y: None, a: self.a, b: self.b };
        }

        // P1 = P2
        // calculate the tangent to the curve at P1
        // & find the point at which the line intersects the curve
        if self == rhs {
            let (x1, y1) = (self.x.unwrap(), self.y.unwrap());
            let m = (3 * x1^2 + self.a) / (2 * y1);
            let x3 = m^2 - 2*x1;
            let y3 = m * (x1 - x3) - y1;
            return Self { x: Some(x3), y: Some(y3), a: self.a, b: self.b };
        }

        // both points are different
        let (x1, y1) = (self.x.unwrap(), self.y.unwrap());
        let (x2, y2) = (rhs.x.unwrap(), rhs.y.unwrap());

        let m = (y2 - y1)/(x2 - x1);
        let x3 = m^2 - x1 - x2;
        let y3 = m * (x1 - x3) - y1;

        Self { x: Some(x3), y: Some(y3), a: self.a, b: self.b }

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn point_outside_curve() {
        Point::from(Some(-1), Some(-2), 5, 7);
    }

    #[test]
    #[ignore = "example point not in curve"]
    fn test_ne() {
        let a = Point::from(Some(3), Some(-7), 5, 7);
        let b = Point::from(Some(18), Some(77), 5, 7);
        assert!(a != b);
    }

    #[test]
    fn test_add0() {
        let a = Point::from(None, None, 5, 7);
        let b = Point::from(Some(2), Some(5), 5, 7);
        let c = Point::from(Some(2), Some(-5), 5, 7);

        assert_eq!(a + b, b);
        assert_eq!(b + a, b);
        assert_eq!(b + c, a);
    }

    #[test]
    fn test_add1() {
        let a = Point::from(Some(3), Some(7), 5, 7);
        let b = Point::from(Some(-1), Some(-1), 5, 7);

        let c = Point::from(Some(2), Some(-5), 5, 7);

        assert_eq!(a + b, c);
    }

    #[test]
    fn test_add2() {
        let a = Point::from(Some(-1), Some(-1), 5, 7);
        let c = Point::from(Some(18), Some(77), 5, 7);

        assert_eq!(a + a, c);
    }

}