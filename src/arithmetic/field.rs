use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

use num_bigint::BigInt;

use crate::errors::FieldError;

#[derive(Debug, Clone)]
pub struct FieldElement {
    num: BigInt,
    pub prime: BigInt,
}

impl FieldElement {
    pub fn from(num: BigInt, prime: BigInt) -> Result<Self, FieldError> {
        // Validate the prime itself
        // still assuming this is a prime
        if prime <= BigInt::from(1u8) {
            return Err(FieldError::InvalidPrime(prime));
        }
        // Normalize num - this ALWAYS produces a result in [0, prime-1]
        // num % prime` gives us something in `[-(prime-1), prime-1]`
        // - Adding `prime` shifts it to `[1, 2*prime-1]`
        // - Taking `% prime` again brings it to `[0, prime-1]
        let normalized = ((num % &prime) + &prime) % &prime;
        Ok(Self {
            num: normalized,
            prime,
        })
    }

    // TODO: fix repeated method
    pub fn pow(&self, exp: &BigInt) -> Self {
        let n: BigInt = exp % (&self.prime - 1_u8);
        let num = self.num.modpow(&n, &self.prime);
        Self {
            num,
            prime: self.prime.clone(),
        }
    }

    pub fn is_odd(&self) -> bool {
        &self.num % 2 == BigInt::from(1_u8)
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
    type Output = Result<Self, FieldError>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            return Err(FieldError::DifferentFields);
        }
        Ok(Self {
            num: (&self.num + &rhs.num) % &self.prime,
            prime: self.prime,
        })
    }
}

impl Sub for FieldElement {
    type Output = Result<Self, FieldError>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            return Err(FieldError::DifferentFields);
        }
        let res = &self.num - &rhs.num;
        Ok(Self {
            num: (res % &self.prime + &self.prime) % &self.prime,
            prime: self.prime,
        })
    }
}

impl Mul for FieldElement {
    type Output = Result<Self, FieldError>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            return Err(FieldError::DifferentFields);
        }
        Ok(Self {
            num: (&self.num * &rhs.num) % &self.prime,
            prime: self.prime,
        })
    }
}

impl Mul<BigInt> for FieldElement {
    type Output = Result<Self, FieldError>;

    fn mul(self, rhs: BigInt) -> Self::Output {
        let new_num = (&self.num * &rhs) % &self.prime;
        Ok(Self {
            num: new_num,
            prime: self.prime,
        })
    }
}

impl Mul<FieldElement> for BigInt {
    type Output = Result<FieldElement, FieldError>;

    fn mul(self, rhs: FieldElement) -> Self::Output {
        let new_num = (&self * &rhs.num) % &rhs.prime;
        Ok(FieldElement {
            num: new_num,
            prime: rhs.prime,
        })
    }
}

impl Div for FieldElement {
    type Output = Result<Self, FieldError>;

    fn div(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            Err(FieldError::DifferentFields)
        } else {
            let exp: BigInt = &self.prime - 2_u8;
            let rhs_inv = rhs.num.modpow(&exp, &self.prime);
            Ok(Self {
                num: (&self.num * &rhs_inv) % &self.prime,
                prime: self.prime,
            })
        }
    }
}

#[cfg(test)]
mod field_elem_tests {
    use super::*;

    #[test]
    fn test_field_elem_eq() {
        let a = FieldElement::from(BigInt::from(7_u8), BigInt::from(13_u8)).unwrap();
        let b = FieldElement::from(BigInt::from(6_u8), BigInt::from(13_u8)).unwrap();

        assert_eq!(a, a);
        assert_ne!(a, b);
    }

    #[test]
    fn test_field_add() {
        let prime = BigInt::from(13_u8);
        let a = FieldElement::from(BigInt::from(7_u8), prime.clone()).unwrap();
        let b = FieldElement::from(BigInt::from(12_u8), prime.clone()).unwrap();
        let c = FieldElement::from(BigInt::from(6_u8), prime.clone()).unwrap();

        assert_eq!((a + b).unwrap(), c);
    }

    #[test]
    fn test_field_sub() {
        let prime = BigInt::from(19_u8);
        let a = FieldElement::from(BigInt::from(6_u8), prime.clone()).unwrap();
        let b = FieldElement::from(BigInt::from(13_u8), prime.clone()).unwrap();
        let c = FieldElement::from(BigInt::from(12_u8), prime.clone()).unwrap();

        assert_eq!((a - b).unwrap(), c);
    }

    #[test]
    fn test_field_mul() {
        let prime = BigInt::from(13_u8);
        let a = FieldElement::from(BigInt::from(3_u8), prime.clone()).unwrap();
        let b = FieldElement::from(BigInt::from(12_u8), prime.clone()).unwrap();
        let c = FieldElement::from(BigInt::from(10_u8), prime.clone()).unwrap();

        assert_eq!((a * b).unwrap(), c);
    }

    #[test]
    fn test_field_pow() {
        let prime = BigInt::from(13_u8);
        let a = FieldElement::from(BigInt::from(3_u8), prime.clone()).unwrap();
        let b = FieldElement::from(BigInt::from(1_u8), prime.clone()).unwrap();

        assert_eq!(a.pow(&BigInt::from(3_u8)), b);
    }
}
