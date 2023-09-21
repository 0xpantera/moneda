use std::fmt::Display;
use std::ops::Add;

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
}
