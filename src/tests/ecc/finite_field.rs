#[cfg(test)]
mod tests {
    use bnum::types::{U256, U512};
    use bnum::BTryFrom;
    use once_cell::sync::Lazy;
    use std::ops::{Add, Div, Mul, Sub};

    field_element!(FieldElement, U256, U512, U256::from_digit(31));

    #[test]
    fn test_ne() {
        let a = FieldElement::new(U256::from_digit(2));
        let b = FieldElement::new(U256::from_digit(2));
        let c = FieldElement::new(U256::from_digit(15));
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_add() {
        let a = FieldElement::new(U256::from_digit(2));
        let b = FieldElement::new(U256::from_digit(15));
        let c = FieldElement::new(U256::from_digit(17));
        assert_eq!(a + b, c);
        let a = FieldElement::new(U256::from_digit(17));
        let b = FieldElement::new(U256::from_digit(21));
        let c = FieldElement::new(U256::from_digit(7));
        assert_eq!(a + b, c);
    }

    #[test]
    fn test_sub() {
        let a = FieldElement::new(U256::from_digit(29));
        let b = FieldElement::new(U256::from_digit(4));
        let c = FieldElement::new(U256::from_digit(25));
        assert_eq!(a - b, c);
        let a = FieldElement::new(U256::from_digit(15));
        let b = FieldElement::new(U256::from_digit(30));
        let c = FieldElement::new(U256::from_digit(16));
        assert_eq!(a - b, c);
    }

    #[test]
    fn test_mul() {
        let a = FieldElement::new(U256::from_digit(24));
        let b = FieldElement::new(U256::from_digit(19));
        let c = FieldElement::new(U256::from_digit(22));
        assert_eq!(a * b, c);
    }

    #[test]
    fn test_pow() {
        let a = FieldElement::new(U256::from_digit(17));
        let b = FieldElement::new(U256::from_digit(15));
        assert_eq!(a.pow(U256::from_digit(3), false), b);
    }

    #[test]
    fn test_div() {
        let a = FieldElement::new(U256::from_digit(3));
        let b = FieldElement::new(U256::from_digit(24));
        let c = FieldElement::new(U256::from_digit(4));
        assert_eq!(a / b, c);
        let a = FieldElement::new(U256::from_digit(17));
        let b = FieldElement::new(U256::from_digit(29));
        assert_eq!(a.pow(U256::from_digit(3), true), b);
    }
}
