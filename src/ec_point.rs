#![allow(dead_code)]

use std::ops::{Add};
use std::fmt::Display;

use crate::field_element::FieldElement;

use num_bigint::{BigInt};

#[derive(Debug, Clone)]
pub struct Point {
    x: Option<FieldElement>,
    y: Option<FieldElement>,
    a: FieldElement,
    b: FieldElement,
}

impl Point {
    pub fn from(x: Option<FieldElement>, y: Option<FieldElement>, a: FieldElement, b: FieldElement) -> Self {
        match (x.clone(), y.clone()) {
            (Some(xs), Some(ys)) => {
                if ys.clone().pow(BigInt::from(2)) != xs.clone().pow(BigInt::from(3)) + a.clone() * xs.clone() + b.clone() {
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
            let m: FieldElement = (BigInt::from(3_u8) * x1.clone().pow(BigInt::from(2_u8)) + self.a.clone()) / (BigInt::from(2_u8) * y1.clone());
            let x3: FieldElement = m.clone().pow(BigInt::from(2_u8)) - BigInt::from(2_u8) * x1.clone();
            let y3 = m * (x1 - x3.clone()) - y1;
            return Self { x: Some(x3), y: Some(y3), a: self.a, b: self.b };
        }

        if self == rhs && self.y == Some(FieldElement::from(BigInt::from(0_u8), self.a.prime.clone())) {
            return Self { x: None, y: None, a: self.a, b: self.b };
        }

        // both points are different
        let (x1, y1) = (self.x.unwrap(), self.y.unwrap());
        let (x2, y2) = (rhs.x.unwrap(), rhs.y.unwrap());

        let m = (y2.clone() - y1.clone())/(x2.clone() - x1.clone());
        let x3 = m.clone().pow(BigInt::from(2_u8)) - x1.clone() - x2.clone();
        let y3 = m * (x1 - x3.clone()) - y1;

        Self { x: Some(x3), y: Some(y3), a: self.a, b: self.b }

    }
}


#[cfg(test)]
mod elliptic_curve_point_tests {
    use super::*;

    #[test]
    #[should_panic]
    fn point_outside_curve() {
        let prime = BigInt::from(223);
        let x = FieldElement::from(BigInt::from(-1), prime.clone());
        let y = FieldElement::from(BigInt::from(-2), prime.clone());
        let a = FieldElement::from(BigInt::from(5), prime.clone());
        let b = FieldElement::from(BigInt::from(7), prime.clone());
        Point::from(
            Some(x), 
            Some(y),
            a,
            b,
        );
    }

    #[test]
    fn test_ne() {
        let prime = BigInt::from(223);
        let x1 = FieldElement::from(BigInt::from(192), prime.clone());
        let y1 = FieldElement::from(BigInt::from(105), prime.clone());

        let x2 = FieldElement::from(BigInt::from(17), prime.clone());
        let y2 = FieldElement::from(BigInt::from(56), prime.clone());

        let a = FieldElement::from(BigInt::from(0), prime.clone());
        let b = FieldElement::from(BigInt::from(7), prime.clone());

        let p1 = Point::from(Some(x1), Some(y1), a.clone(), b.clone());
        let p2 = Point::from(Some(x2), Some(y2), a.clone(), b.clone());
        assert!(p1 != p2);
    }

    #[test]
    fn test_add0() {
        let prime = BigInt::from(223);
        let x1 = FieldElement::from(BigInt::from(192), prime.clone());
        let y1 = FieldElement::from(BigInt::from(105), prime.clone());

        let x2 = FieldElement::from(BigInt::from(17), prime.clone());
        let y2 = FieldElement::from(BigInt::from(56), prime.clone());

        let a = FieldElement::from(BigInt::from(0), prime.clone());
        let b = FieldElement::from(BigInt::from(7), prime.clone());

        let p1 = Point::from(None, None, a.clone(), b.clone());
        let p2 = Point::from(Some(x1), Some(y1), a.clone(), b.clone());
        let p3 = Point::from(Some(x2), Some(y2), a.clone(), b.clone());

        assert_eq!(p1.clone() + p2.clone(), p2.clone());
        assert_eq!(p2.clone() + p1.clone(), p2.clone());
    }

    #[test]
    fn test_add1() {
        let prime = BigInt::from(223);
        let x1 = FieldElement::from(BigInt::from(192), prime.clone());
        let y1 = FieldElement::from(BigInt::from(105), prime.clone());

        let x2 = FieldElement::from(BigInt::from(17), prime.clone());
        let y2 = FieldElement::from(BigInt::from(56), prime.clone());

        let x3 = FieldElement::from(BigInt::from(170), prime.clone());
        let y3 = FieldElement::from(BigInt::from(142), prime.clone());

        let a = FieldElement::from(BigInt::from(0), prime.clone());
        let b = FieldElement::from(BigInt::from(7), prime.clone());

        let p1 = Point::from(Some(x1), Some(y1), a.clone(), b.clone());
        let p2 = Point::from(Some(x2), Some(y2), a.clone(), b.clone());

        let p3 = Point::from(Some(x3), Some(y3), a.clone(), b.clone());

        assert_eq!(p1 + p2, p3);
    }

    #[test]
    fn test_add2() {
        let prime = BigInt::from(223);
        let x1 = FieldElement::from(BigInt::from(17), prime.clone());
        let y1 = FieldElement::from(BigInt::from(56), prime.clone());

        let x2 = FieldElement::from(BigInt::from(13), prime.clone());
        let y2 = FieldElement::from(BigInt::from(190), prime.clone());

        let a = FieldElement::from(BigInt::from(0), prime.clone());
        let b = FieldElement::from(BigInt::from(7), prime.clone());

        let p1 = Point::from(Some(x1), Some(y1), a.clone(), b.clone());
        let p2 = Point::from(Some(x2), Some(y2), a.clone(), b.clone());

        assert_eq!(p1.clone() + p1.clone(), p2);
    }

}