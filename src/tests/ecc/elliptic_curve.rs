use crate::ecc::elliptic_curve::EllipticCurve;
use crate::ecc::finite_field::{FieldElement, Modulus, Sqrt};
use bnum::BUint;
use once_cell::sync::Lazy;

type U64 = BUint<1>;

field_element!(Field223, P223, 64, "223", 10, tonelli_shanks);

point!(F223Point07, EC07, P223, 64, "0", "7", "0", "0", "223", 10);

point!(F223Point57, EC57, P223, 64, "5", "7", "0", "0", "223", 10);

#[test]
fn test_ne() {
    let a = F223Point57::from_values(U64::from_digit(3), U64::from_digit(7));
    let b = F223Point57::from_values(U64::from_digit(18), U64::from_digit(77));
    assert_ne!(a, b);
}

#[test]
fn test_on_curve() {
    let valid_points = [(192, 105), (17, 56), (1, 193)];
    let invalid_points = [(200, 119), (42, 99)];
    for (x, y) in valid_points {
        assert!(F223Point07::from_values(U64::from_digit(x), U64::from_digit(y)).is_ok());
    }
    for (x, y) in invalid_points {
        assert!(F223Point07::from_values(U64::from_digit(x), U64::from_digit(y)).is_err());
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
        let p1 = F223Point07::from_values(U64::from_digit(x1), U64::from_digit(y1)).unwrap();
        let p2 = F223Point07::from_values(U64::from_digit(x2), U64::from_digit(y2)).unwrap();
        let p3 = F223Point07::from_values(U64::from_digit(x3), U64::from_digit(y3)).unwrap();
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
        let p1 = F223Point07::from_values(U64::from_digit(x1), U64::from_digit(y1)).unwrap();
        let p2 = F223Point07::from_values(U64::from_digit(x2), U64::from_digit(y2)).unwrap();
        assert_eq!(p1 * U64::from_digit(coef), p2);
    }
    let p1 = F223Point07::from_values(U64::from_digit(47), U64::from_digit(71)).unwrap();
    let p2 = F223Point07::INFINITY;
    assert_eq!(p1 * U64::from_digit(21), p2);
}
