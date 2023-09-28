use crate::ec_point::Point;
use crate::field_element::FieldElement;
use num_bigint::BigUint;

/*
mod tests {
    use super::*;

    #[test]
    #[ignore = "reason"]
    fn test_on_curve() {
        let prime = BigUint::from(223_u16);
        let a = FieldElement::from(BigUint::from(0_u8), prime);
        let b = FieldElement::from(BigUint::from(7_u8), prime);

        let valid_points = vec![(192, 105), (17, 56), (1, 193)];
        let invalid_points = ((200, 119), (42, 99));

        for (x_raw, y_raw) in valid_points.iter() {
            let x = FieldElement::from(BigUint::from(*x_raw as u16), prime);
            let y = FieldElement::from(BigUint::from(*y_raw as u16), prime);
            Point::from(Some(x), Some(y), a, b);
        }
    }
}
*/