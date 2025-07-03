use std::fmt::Display;
use std::ops::{Add, Mul};

use crate::arithmetic::field::FieldElement;
use crate::errors::PointError;

use num_bigint::BigInt;

#[derive(Debug, Clone)]
pub enum Point {
    Infinity {
        a: FieldElement,
        b: FieldElement,
    },
    Finite {
        x: FieldElement,
        y: FieldElement,
        a: FieldElement,
        b: FieldElement,
    },
}

impl Point {
    pub fn finite(
        x: FieldElement,
        y: FieldElement,
        a: FieldElement,
        b: FieldElement,
    ) -> Result<Self, PointError> {
        // y^2 = x^3 + ax + b
        let lhs = y.pow(&BigInt::from(2));
        let x_cubed = x.pow(&BigInt::from(3));
        let ax = (a.clone() * x.clone())?;
        let rhs = ((x_cubed + ax)? + b.clone())?;

        if lhs != rhs {
            return Err(PointError::NotOnCurve {
                x: format!("{}", x),
                y: format!("{}", y),
            });
        }

        Ok(Point::Finite { x, y, a, b })
    }

    pub fn infinity(a: FieldElement, b: FieldElement) -> Self {
        Point::Infinity { a, b }
    }

    fn add_identity(self, other: Point) -> Point {
        match (self, other) {
            (Point::Infinity { .. }, p) => p,
            (p, Point::Infinity { .. }) => p,
            _ => unreachable!("add_identity called with two finite points"),
        }
    }

    fn add_inverse_points(self, _other: Point) -> Point {
        match self {
            Point::Finite { a, b, .. } => Point::Infinity { a, b },
            Point::Infinity { a, b } => Point::Infinity { a, b },
        }
    }

    fn point_doubling(self) -> Result<Point, PointError> {
        match self {
            Point::Finite { x, y, a, b } => {
                // Check for special case: y = 0 (tangent is vertical)
                if y == FieldElement::from(BigInt::from(0), y.prime.clone()).unwrap() {
                    return Ok(Point::Infinity { a, b });
                }

                // Calculate slope: m = (3x² + a) / (2y)
                let three_x_squared = (BigInt::from(3) * x.pow(&BigInt::from(2)))?;
                let numerator = (three_x_squared + a.clone())?;
                let denominator = (BigInt::from(2) * y.clone())?;
                let m = (numerator / denominator)?;

                // Calculate new coordinates
                let m_squared = m.pow(&BigInt::from(2));
                let two_x = (BigInt::from(2) * x.clone())?;
                let x3 = (m_squared - two_x)?;

                let x_minus_x3 = (x - x3.clone())?;
                let m_times_diff = (m * x_minus_x3)?;
                let y3 = (m_times_diff - y)?;

                Ok(Point::Finite { x: x3, y: y3, a, b })
            }
            Point::Infinity { a, b } => Ok(Point::Infinity { a, b }),
        }
    }

    fn add_different_points(self, other: Point) -> Result<Point, PointError> {
        match (self, other) {
            (Point::Finite { x: x1, y: y1, a, b }, Point::Finite { x: x2, y: y2, .. }) => {
                // Calculate slope: m = (y2 - y1) / (x2 - x1)
                let y_diff = (y2.clone() - y1.clone())?;
                let x_diff = (x2.clone() - x1.clone())?;
                let m = (y_diff / x_diff)?;

                // Calculate new coordinates
                let m_squared = m.pow(&BigInt::from(2));
                let x3 = ((m_squared - x1.clone())? - x2.clone())?;

                let x1_minus_x3 = (x1 - x3.clone())?;
                let m_times_diff = (m * x1_minus_x3)?;
                let y3 = (m_times_diff - y1)?;

                Ok(Point::Finite { x: x3, y: y3, a, b })
            }
            _ => unreachable!("add_different_points called with infinity points"),
        }
    }

    fn is_inverse_of(&self, other: &Point) -> bool {
        match (self, other) {
            (
                Point::Finite {
                    x: x1,
                    y: y1,
                    a: a1,
                    b: b1,
                },
                Point::Finite {
                    x: x2,
                    y: y2,
                    a: a2,
                    b: b2,
                },
            ) => a1 == a2 && b1 == b2 && x1 == x2 && y1 != y2,
            _ => false,
        }
    }

    fn is_same_point(&self, other: &Point) -> bool {
        match (self, other) {
            (
                Point::Finite {
                    x: x1,
                    y: y1,
                    a: a1,
                    b: b1,
                },
                Point::Finite {
                    x: x2,
                    y: y2,
                    a: a2,
                    b: b2,
                },
            ) => a1 == a2 && b1 == b2 && x1 == x2 && y1 == y2,
            (Point::Infinity { a: a1, b: b1 }, Point::Infinity { a: a2, b: b2 }) => {
                a1 == a2 && b1 == b2
            }
            _ => false,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Two finite points: all coordinates and curve params must match
            (
                Point::Finite {
                    x: x1,
                    y: y1,
                    a: a1,
                    b: b1,
                },
                Point::Finite {
                    x: x2,
                    y: y2,
                    a: a2,
                    b: b2,
                },
            ) => x1 == x2 && y1 == y2 && a1 == a2 && b1 == b2,
            // Two infinity points: curve params must match
            // The point at infinity belongs to a specific curve.
            // O₁ (infinity on BN254) ≠ O₂ (infinity on BLS12-381).
            (Point::Infinity { a: a1, b: b1 }, Point::Infinity { a: a2, b: b2 }) => {
                a1 == a2 && b1 == b2
            }
            // Finite vs Infinity: never equal
            _ => false,
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Point::Finite { x, y, a, b } => {
                write!(f, "({}, {}) on y² = x³ + {}x + {}", x, y, a, b)
            }
            Point::Infinity { a, b } => {
                write!(f, "O on y² = x³ + {}x + {}", a, b)
            }
        }
    }
}

impl Add for Point {
    type Output = Result<Self, PointError>;

    fn add(self, other: Self) -> Self::Output {
        use Point::*;

        match (&self, &other) {
            (Infinity { .. }, _) => Ok(self.add_identity(other)),
            (_, Infinity { .. }) => Ok(other.add_identity(self)),
            (Finite { .. }, Finite { .. }) => {
                if self.is_inverse_of(&other) {
                    Ok(self.add_inverse_points(other))
                } else if self.is_same_point(&other) {
                    self.point_doubling()
                } else {
                    self.add_different_points(other)
                }
            }
        }
    }
}

impl Mul<Point> for BigInt {
    type Output = Result<Point, PointError>;

    fn mul(self, rhs: Point) -> Self::Output {
        let mut coef = self;
        let mut current = rhs.clone();

        // Get curve parameters and create identity element
        let (a, b) = match &rhs {
            Point::Finite { a, b, .. } => (a, b),
            Point::Infinity { a, b } => (a, b),
        };

        let mut res = Point::Infinity {
            a: a.clone(),
            b: b.clone(),
        };

        while coef > BigInt::from(0) {
            // check if coefficient is odd
            if &coef % 2_u8 == BigInt::from(1_u8) {
                res = (res + current.clone())?;
            }
            current = (current.clone() + current)?;
            coef >>= 1;
        }
        Ok(res)
    }
}

#[cfg(test)]
mod elliptic_curve_point_tests {
    use super::*;

    #[test]
    #[should_panic]
    fn point_outside_curve() {
        let prime = BigInt::from(223);
        let x = FieldElement::from(BigInt::from(-1), prime.clone()).unwrap();
        let y = FieldElement::from(BigInt::from(-2), prime.clone()).unwrap();
        let a = FieldElement::from(BigInt::from(5), prime.clone()).unwrap();
        let b = FieldElement::from(BigInt::from(7), prime.clone()).unwrap();
        Point::finite(x, y, a, b).unwrap();
    }

    #[test]
    fn test_ne() {
        let prime = BigInt::from(223);
        let x1 = FieldElement::from(BigInt::from(192), prime.clone()).unwrap();
        let y1 = FieldElement::from(BigInt::from(105), prime.clone()).unwrap();

        let x2 = FieldElement::from(BigInt::from(17), prime.clone()).unwrap();
        let y2 = FieldElement::from(BigInt::from(56), prime.clone()).unwrap();

        let a = FieldElement::from(BigInt::from(0), prime.clone()).unwrap();
        let b = FieldElement::from(BigInt::from(7), prime.clone()).unwrap();

        let p1 = Point::finite(x1, y1, a.clone(), b.clone()).unwrap();
        let p2 = Point::finite(x2, y2, a.clone(), b.clone()).unwrap();
        assert!(p1 != p2);
    }

    #[test]
    fn test_add0() {
        let prime = BigInt::from(223);
        let x1 = FieldElement::from(BigInt::from(192), prime.clone()).unwrap();
        let y1 = FieldElement::from(BigInt::from(105), prime.clone()).unwrap();

        let a = FieldElement::from(BigInt::from(0), prime.clone()).unwrap();
        let b = FieldElement::from(BigInt::from(7), prime.clone()).unwrap();

        let infinity = Point::infinity(a.clone(), b.clone());
        let p1 = Point::finite(x1, y1, a.clone(), b.clone()).unwrap();

        assert_eq!((infinity.clone() + p1.clone()).unwrap(), p1.clone());
        assert_eq!((p1.clone() + infinity.clone()).unwrap(), p1.clone());
    }

    #[test]
    fn test_add1() {
        let prime = BigInt::from(223);
        let x1 = FieldElement::from(BigInt::from(192), prime.clone()).unwrap();
        let y1 = FieldElement::from(BigInt::from(105), prime.clone()).unwrap();

        let x2 = FieldElement::from(BigInt::from(17), prime.clone()).unwrap();
        let y2 = FieldElement::from(BigInt::from(56), prime.clone()).unwrap();

        let x3 = FieldElement::from(BigInt::from(170), prime.clone()).unwrap();
        let y3 = FieldElement::from(BigInt::from(142), prime.clone()).unwrap();

        let a = FieldElement::from(BigInt::from(0), prime.clone()).unwrap();
        let b = FieldElement::from(BigInt::from(7), prime.clone()).unwrap();

        let p1 = Point::finite(x1, y1, a.clone(), b.clone()).unwrap();
        let p2 = Point::finite(x2, y2, a.clone(), b.clone()).unwrap();
        let p3 = Point::finite(x3, y3, a.clone(), b.clone()).unwrap();

        assert_eq!((p1 + p2).unwrap(), p3);
    }

    #[test]
    fn test_add2() {
        let prime = BigInt::from(223);
        let x1 = FieldElement::from(BigInt::from(17), prime.clone()).unwrap();
        let y1 = FieldElement::from(BigInt::from(56), prime.clone()).unwrap();

        let x2 = FieldElement::from(BigInt::from(13), prime.clone()).unwrap();
        let y2 = FieldElement::from(BigInt::from(190), prime.clone()).unwrap();

        let a = FieldElement::from(BigInt::from(0), prime.clone()).unwrap();
        let b = FieldElement::from(BigInt::from(7), prime.clone()).unwrap();

        let p1 = Point::finite(x1, y1, a.clone(), b.clone()).unwrap();
        let p2 = Point::finite(x2, y2, a.clone(), b.clone()).unwrap();

        assert_eq!((p1.clone() + p1.clone()).unwrap(), p2);
    }

    #[test]
    fn test_add3() {
        let prime = BigInt::from(223);
        let x1 = FieldElement::from(BigInt::from(170), prime.clone()).unwrap();
        let y1 = FieldElement::from(BigInt::from(142), prime.clone()).unwrap();

        let x2 = FieldElement::from(BigInt::from(60), prime.clone()).unwrap();
        let y2 = FieldElement::from(BigInt::from(139), prime.clone()).unwrap();

        let x3 = FieldElement::from(BigInt::from(220), prime.clone()).unwrap();
        let y3 = FieldElement::from(BigInt::from(181), prime.clone()).unwrap();

        let a = FieldElement::from(BigInt::from(0), prime.clone()).unwrap();
        let b = FieldElement::from(BigInt::from(7), prime.clone()).unwrap();

        let p1 = Point::finite(x1, y1, a.clone(), b.clone()).unwrap();
        let p2 = Point::finite(x2, y2, a.clone(), b.clone()).unwrap();
        let p3 = Point::finite(x3, y3, a.clone(), b.clone()).unwrap();

        assert_eq!((p1 + p2).unwrap(), p3);
    }

    #[test]
    fn test_add4() {
        let prime = BigInt::from(223);
        let x1 = FieldElement::from(BigInt::from(47), prime.clone()).unwrap();
        let y1 = FieldElement::from(BigInt::from(71), prime.clone()).unwrap();

        let x2 = FieldElement::from(BigInt::from(17), prime.clone()).unwrap();
        let y2 = FieldElement::from(BigInt::from(56), prime.clone()).unwrap();

        let x3 = FieldElement::from(BigInt::from(215), prime.clone()).unwrap();
        let y3 = FieldElement::from(BigInt::from(68), prime.clone()).unwrap();

        let a = FieldElement::from(BigInt::from(0), prime.clone()).unwrap();
        let b = FieldElement::from(BigInt::from(7), prime.clone()).unwrap();

        let p1 = Point::finite(x1, y1, a.clone(), b.clone()).unwrap();
        let p2 = Point::finite(x2, y2, a.clone(), b.clone()).unwrap();
        let p3 = Point::finite(x3, y3, a.clone(), b.clone()).unwrap();

        assert_eq!((p1 + p2).unwrap(), p3);
    }

    #[test]
    fn test_add5() {
        let prime = BigInt::from(223);
        let x1 = FieldElement::from(BigInt::from(143), prime.clone()).unwrap();
        let y1 = FieldElement::from(BigInt::from(98), prime.clone()).unwrap();

        let x2 = FieldElement::from(BigInt::from(76), prime.clone()).unwrap();
        let y2 = FieldElement::from(BigInt::from(66), prime.clone()).unwrap();

        let x3 = FieldElement::from(BigInt::from(47), prime.clone()).unwrap();
        let y3 = FieldElement::from(BigInt::from(71), prime.clone()).unwrap();

        let a = FieldElement::from(BigInt::from(0), prime.clone()).unwrap();
        let b = FieldElement::from(BigInt::from(7), prime.clone()).unwrap();

        let p1 = Point::finite(x1, y1, a.clone(), b.clone()).unwrap();
        let p2 = Point::finite(x2, y2, a.clone(), b.clone()).unwrap();
        let p3 = Point::finite(x3, y3, a.clone(), b.clone()).unwrap();

        assert_eq!((p1 + p2).unwrap(), p3);
    }

    #[test]
    fn test_scalar_mul() {
        let prime = BigInt::from(223);
        let x1 = FieldElement::from(BigInt::from(47), prime.clone()).unwrap();
        let y1 = FieldElement::from(BigInt::from(71), prime.clone()).unwrap();

        let x2 = FieldElement::from(BigInt::from(194), prime.clone()).unwrap();
        let y2 = FieldElement::from(BigInt::from(172), prime.clone()).unwrap();

        let a = FieldElement::from(BigInt::from(0), prime.clone()).unwrap();
        let b = FieldElement::from(BigInt::from(7), prime.clone()).unwrap();

        let p1 = Point::finite(x1, y1, a.clone(), b.clone()).unwrap();
        let p2 = Point::finite(x2, y2, a.clone(), b.clone()).unwrap();
        let s = BigInt::from(17);

        assert_eq!((s * p1).unwrap(), p2);
    }
}
