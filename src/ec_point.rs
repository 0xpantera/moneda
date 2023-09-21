#![allow(dead_code)]
use std::ops::{Add};
use std::fmt::Display;

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

        if self.x.is_none() { return rhs; }
        if rhs.x.is_none() { return self; }

        if self.x == rhs.x && self.y != rhs.y {
            return Self { x: None, y: None, a: self.a, b: self.b };
        }

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
        let p1 = Point::from(Some(-1), Some(-2), 5, 7);
    }

}