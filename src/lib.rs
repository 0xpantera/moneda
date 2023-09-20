use std::fmt::Display;

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
}
