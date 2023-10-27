#![allow(dead_code)]

use std::fmt::Display;
use std::ops::{Add, Sub, Mul, Div};

use num_bigint::{BigInt};
use num_traits::cast::ToPrimitive;

#[derive(Debug, Clone)]
pub struct FieldElement {
    num: BigInt,
    pub prime: BigInt,
}

impl FieldElement {
    pub fn from(num: BigInt, prime: BigInt) -> Self {
        if num >= prime || num < BigInt::from(0_u8) {
           panic!("Num {} not in field of order {}", num, prime);
        }
        Self { num, prime }
    }

    pub fn pow(self, exp: BigInt) -> Self {
        let n: BigInt = exp % (self.prime.clone() - 1_u8);
        let num = self.num.pow(n.try_into().unwrap()) % self.prime.clone();
        Self {
            num,
            prime: self.prime,
        }
    }

    pub fn is_odd(&self) -> bool {
        self.num.to_i64().unwrap() % 2 != 0
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.prime == other.prime
    }
}

impl Display for FieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FieldElement_{}({})", self.prime, self.num)
    }
}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("Elements must be in the same field")
        }
        Self {
            num: (self.num + rhs.num) % self.prime.clone(),
            prime: self.prime,
        }
    }   
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("Elements must be in the same field")
        }
        let res = self.num - rhs.num;
        Self {
            num: (res % self.prime.clone() + self.prime.clone()) % self.prime.clone(),
            prime: self.prime,
        }
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("Elements must be in the same field")
        }
        Self {
            num: self.num * rhs.num % self.prime.clone(),
            prime: self.prime,
        }
    }
}

impl Mul<BigInt> for FieldElement {
    type Output = Self;

    fn mul(self, rhs: BigInt) -> Self::Output {
        let new_num = (self.num.clone() * rhs) % &self.prime;
        FieldElement {
            num: new_num,
            prime: self.prime,
        }
    }
}

impl Mul<FieldElement> for BigInt {
    type Output = FieldElement;

    fn mul(self, rhs: FieldElement) -> Self::Output {
        let new_num = (self * rhs.num.clone()) % &rhs.prime;
        FieldElement {
            num: new_num,
            prime: rhs.prime,
        }
    }
}

impl Div for FieldElement {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("Elements must be in the same field")
        }
        let res: u32 = (self.prime.clone() - 2_u8).try_into().unwrap();
        Self {
            num: self.num * rhs.num.pow(res) % self.prime.clone(),
            prime: self.prime,
        }
    }
}

#[cfg(test)]
mod field_elem_tests {
    use super::*;

    #[test]
    fn test_field_elem_eq() {
        let a = FieldElement::from(BigInt::from(7_u8), BigInt::from(13_u8));
        let b = FieldElement::from(BigInt::from(6_u8), BigInt::from(13_u8));

        assert_eq!(a, a);
        assert_ne!(a, b);
    }

    #[test]
    fn test_field_add() {
        let prime = BigInt::from(13_u8);
        let a = FieldElement::from(BigInt::from(7_u8), prime.clone());
        let b = FieldElement::from(BigInt::from(12_u8), prime.clone());
        let c = FieldElement::from(BigInt::from(6_u8), prime.clone());

        assert_eq!(a + b, c);
    }

    #[test]
    fn test_field_sub() {
        let prime = BigInt::from(19_u8);
        let a = FieldElement::from(BigInt::from(6_u8), prime.clone());
        let b = FieldElement::from(BigInt::from(13_u8), prime.clone());
        let c = FieldElement::from(BigInt::from(12_u8), prime.clone());

        assert_eq!(a - b, c);
    }

    #[test]
    fn test_field_mul() {
        let prime = BigInt::from(13_u8);
        let a = FieldElement::from(BigInt::from(3_u8), prime.clone());
        let b = FieldElement::from(BigInt::from(12_u8), prime.clone());
        let c = FieldElement::from(BigInt::from(10_u8), prime.clone());

        assert_eq!(a * b, c);
    }

    #[test]
    fn test_field_pow() {
        let prime = BigInt::from(13_u8);
        let a = FieldElement::from(BigInt::from(3_u8), prime.clone());
        let b = FieldElement::from(BigInt::from(1_u8), prime.clone());

        assert_eq!(a.pow(BigInt::from(3_u8)), b);
    }
}
