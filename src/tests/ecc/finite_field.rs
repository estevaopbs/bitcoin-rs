use crate::ecc::finite_field::{Modulus, Sqrt};
use bnum::BUint;

type U64 = BUint<1>;

field_element!(FieldP31, P31, 64, "31", 10, p3mod4);

#[test]
fn test_ne() {
    let a = FieldP31::new(U64::from_digit(2));
    let b = FieldP31::new(U64::from_digit(2));
    let c = FieldP31::new(U64::from_digit(15));
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn test_add() {
    let a = FieldP31::new(U64::from_digit(2));
    let b = FieldP31::new(U64::from_digit(15));
    let c = FieldP31::new(U64::from_digit(17));
    assert_eq!(a + b, c);
    let a = FieldP31::new(U64::from_digit(17));
    let b = FieldP31::new(U64::from_digit(21));
    let c = FieldP31::new(U64::from_digit(7));
    assert_eq!(a + b, c);
}

#[test]
fn test_sub() {
    let a = FieldP31::new(U64::from_digit(29));
    let b = FieldP31::new(U64::from_digit(4));
    let c = FieldP31::new(U64::from_digit(25));
    assert_eq!(a - b, c);
    let a = FieldP31::new(U64::from_digit(15));
    let b = FieldP31::new(U64::from_digit(30));
    let c = FieldP31::new(U64::from_digit(16));
    assert_eq!(a - b, c);
}

#[test]
fn test_mul() {
    let a = FieldP31::new(U64::from_digit(24));
    let b = FieldP31::new(U64::from_digit(19));
    let c = FieldP31::new(U64::from_digit(22));
    assert_eq!(a * b, c);
}

#[test]
fn test_pow() {
    let a = FieldP31::new(U64::from_digit(17));
    let b = FieldP31::new(U64::from_digit(15));
    assert_eq!(a.pow(U64::from_digit(3), false), b);
}

#[test]
fn test_div() {
    let a = FieldP31::new(U64::from_digit(3));
    let b = FieldP31::new(U64::from_digit(24));
    let c = FieldP31::new(U64::from_digit(4));
    assert_eq!(a / b, c);
    let a = FieldP31::new(U64::from_digit(17));
    let b = FieldP31::new(U64::from_digit(29));
    assert_eq!(a.pow(U64::from_digit(3), true), b);
}
