#[cfg(test)]
mod tests {
    use bnum::types::{U256, U512};
    use bnum::BTryFrom;
    use once_cell::sync::Lazy;
    use std::ops::{Add, AddAssign, Div, Mul, Sub};

    field_element!(FieldElement, U256, U512, U256::from_digit(223));

    impl Sqrt for FieldElement {
        fn sqrt(&self) -> Self {
            FieldElement::new(U256::ZERO)
        }
    }

    signature!(Signature, FieldElement, U256);

    point!(
        Point,
        FieldElement,
        Signature,
        U256,
        U256::ZERO,
        U256::SEVEN,
        U256::ZERO,
        U256::ZERO,
        U256::from_digit(223)
    );

    #[test]
    fn test_ne() {
        let a = Point::from_values(U256::from_digit(3), U256::from_digit(7));
        let b = Point::from_values(U256::from_digit(18), U256::from_digit(77));
        assert_ne!(a, b);
    }

    #[test]
    fn test_on_curve() {
        let valid_points = [(192, 105), (17, 56), (1, 193)];
        let invalid_points = [(200, 119), (42, 99)];
        for (x, y) in valid_points {
            assert!(Point::from_values(U256::from_digit(x), U256::from_digit(y)).is_ok());
        }
        for (x, y) in invalid_points {
            assert!(Point::from_values(U256::from_digit(x), U256::from_digit(y)).is_err());
        }
    }

    #[test]
    fn test_add() {
        let additions = [
            (192, 105, 17, 56, 170, 142),
            (47, 71, 117, 141, 60, 139),
            (143, 98, 76, 66, 47, 71),
        ];
        for (x1, y1, x2, y2, x3, y3) in additions {
            let p1 = Point::from_values(U256::from_digit(x1), U256::from_digit(y1)).unwrap();
            let p2 = Point::from_values(U256::from_digit(x2), U256::from_digit(y2)).unwrap();
            let p3 = Point::from_values(U256::from_digit(x3), U256::from_digit(y3)).unwrap();
            assert_eq!(p1 + p2, p3);
        }
    }

    #[test]
    fn test_mul() {
        let multiplications = [
            (2, 192, 105, 49, 71),
            (2, 143, 98, 64, 168),
            (2, 47, 71, 36, 111),
            (4, 47, 71, 194, 51),
            (8, 47, 71, 116, 55),
        ];
        for (coef, x1, y1, x2, y2) in multiplications {
            let p1 = Point::from_values(U256::from_digit(x1), U256::from_digit(y1)).unwrap();
            let p2 = Point::from_values(U256::from_digit(x2), U256::from_digit(y2)).unwrap();
            assert_eq!(p1 * U256::from_digit(coef), p2);
        }
        let p1 = Point::from_values(U256::from_digit(47), U256::from_digit(71)).unwrap();
        let p2 = Point::INFINITY;
        assert_eq!(p1 * U256::from_digit(21), p2);
    }
}
