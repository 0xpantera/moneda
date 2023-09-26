#![allow(dead_code)]

use std::ops::{Add};
use std::fmt::Display;

use num_bigint::{BigInt, ToBigInt};

#[derive(Debug, Clone)]
struct Point {
    x: Option<BigInt>,
    y: Option<BigInt>,
    a: BigInt,
    b: BigInt,
}

impl Point {
    fn from(x: Option<BigInt>, y: Option<BigInt>, a: BigInt, b: BigInt) -> Self {
        match (x.clone(), y.clone()) {
            (Some(xs), Some(ys)) => {
                if ys.pow(2) != xs.pow(3) + a.clone() * xs.clone() + b.clone() {
                    panic!("({}, {}) is not on the curve", xs, ys);
                }
            },
            (Some(xs), None) => panic!("({}, None) is not valid", xs),
            (None, Some(ys)) => panic!("(None, {}) is not valid", ys),
            (None, None) => {},
        }
        Self { x, y, a, b, }
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
            let m: BigInt = (3 * x1.clone().pow(2) + self.a.clone()) / (2 * y1.clone());
            let x3: BigInt = m.clone().pow(2) - 2 * x1.clone();
            let y3 = m * (x1 - x3.clone()) - y1;
            return Self { x: Some(x3), y: Some(y3), a: self.a, b: self.b };
        }

        if self == rhs && self.y == Some(BigInt::from(0)) {
            return Self { x: None, y: None, a: self.a, b: self.b };
        }

        // both points are different
        let (x1, y1) = (self.x.unwrap(), self.y.unwrap());
        let (x2, y2) = (rhs.x.unwrap(), rhs.y.unwrap());

        let m = (y2.clone() - y1.clone())/(x2.clone() - x1.clone());
        let x3 = m.clone().pow(2) - x1.clone() - x2.clone();
        let y3 = m * (x1 - x3.clone()) - y1;

        Self { x: Some(x3), y: Some(y3), a: self.a, b: self.b }

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn point_outside_curve() {
        Point::from(
            Some(BigInt::from(-1)), 
            Some(BigInt::from(-2)),
            BigInt::from(5), 
            BigInt::from(7)
        );
    }

    #[test]
    fn test_ne() {
        let x1 = BigInt::from(3);
        let y1 = BigInt::from(-7);

        let x2 = BigInt::from(18);
        let y2 = BigInt::from(77);

        let a = BigInt::from(5);
        let b = BigInt::from(7);

        let p1 = Point::from(Some(x1), Some(y1), a.clone(), b.clone());
        let p2 = Point::from(Some(x2), Some(y2), a.clone(), b.clone());
        assert!(p1 != p2);
    }

    #[test]
    fn test_add0() {
        let x1 = BigInt::from(2);
        let y1 = BigInt::from(5);

        let x2 = BigInt::from(2);
        let y2 = BigInt::from(-5);

        let a = BigInt::from(5);
        let b = BigInt::from(7);

        let p1 = Point::from(None, None, a.clone(), b.clone());
        let p2 = Point::from(Some(x1), Some(y1), a.clone(), b.clone());
        let p3 = Point::from(Some(x2), Some(y2), a.clone(), b.clone());

        assert_eq!(p1.clone() + p2.clone(), p2.clone());
        assert_eq!(p2.clone() + p1.clone(), p2.clone());
        assert_eq!(p2.clone() + p3.clone(), p1.clone());
    }

    #[test]
    fn test_add1() {
        let x1 = BigInt::from(3);
        let y1 = BigInt::from(7);

        let x2 = BigInt::from(-1);
        let y2 = BigInt::from(-1);

        let x3 = BigInt::from(2);
        let y3 = BigInt::from(-5);

        let a = BigInt::from(5);
        let b = BigInt::from(7);

        let p1 = Point::from(Some(x1), Some(y1), a.clone(), b.clone());
        let p2 = Point::from(Some(x2), Some(y2), a.clone(), b.clone());

        let p3 = Point::from(Some(x3), Some(y3), a.clone(), b.clone());

        assert_eq!(p1 + p2, p3);
    }

    #[test]
    fn test_add2() {
        let x1 = BigInt::from(-1);
        let y1 = BigInt::from(-1);

        let x2 = BigInt::from(18);
        let y2 = BigInt::from(77);

        let a = BigInt::from(5);
        let b = BigInt::from(7);

        let p1 = Point::from(Some(x1), Some(y1), a.clone(), b.clone());
        let p2 = Point::from(Some(x2), Some(y2), a.clone(), b.clone());

        assert_eq!(p1.clone() + p1.clone(), p2);
    }

}