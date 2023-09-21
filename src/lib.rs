use std::fmt::Display;
use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug)]
struct FieldElement {
    num: i128,
    prime: i128,
}

impl FieldElement {
    pub fn from(num: i128, prime: i128) -> Self {
        if num >= prime || num < 0 {
           panic!("Num {} not in field of order {}", num, prime);
        }
        Self { num, prime }
    }

    pub fn pow(self, exp: i32) -> Self {
        let n = exp % (self.prime as i32 - 1);
        let num = self.num.pow(n.try_into().unwrap()) % self.prime;
        Self {
            num,
            prime: self.prime,
        }
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
            num: (self.num + rhs.num) % self.prime,
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
            num: (res % self.prime + self.prime) % self.prime,
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
            num: self.num * rhs.num % self.prime,
            prime: self.prime,
        }
    }
}

impl Div for FieldElement {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("Elements must be in the same field")
        }
        let res: u32 = (self.prime - 2).try_into().unwrap();
        Self {
            num: self.num * rhs.num.pow(res) % self.prime,
            prime: self.prime,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_elem_eq() {
        let a = FieldElement::from(7, 13);
        let b = FieldElement::from(6, 13);

        assert_eq!(a, a);
        assert_ne!(a, b);
    }

    #[test]
    fn test_field_add() {
        let a = FieldElement::from(7, 13);
        let b = FieldElement::from(12, 13);
        let c = FieldElement::from(6, 13);

        assert_eq!(a + b, c);
    }

    #[test]
    fn test_field_sub() {
        let a = FieldElement::from(6, 19);
        let b = FieldElement::from(13, 19);
        let c = FieldElement::from(12, 19);

        assert_eq!(a - b, c);
    }

    #[test]
    fn test_field_mul() {
        let a = FieldElement::from(3, 13);
        let b = FieldElement::from(12, 13);
        let c = FieldElement::from(10, 13);

        assert_eq!(a * b, c);
    }

    #[test]
    fn test_field_pow() {
        let a = FieldElement::from(3, 13);
        let b = FieldElement::from(1, 13);

        assert_eq!(a.pow(3), b);
    }
}
