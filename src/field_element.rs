use std::ops::{Add, Div, Mul, Sub};

fn mod_pow(mut base: i32, mut exp: i32, modulus: i32) -> i32 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus;
    }
    result
}

#[derive(Debug)]
pub struct FieldElement {
    num: i32,
    prime: i32,
}

impl FieldElement {
    fn new(num: i32, prime: i32) -> FieldElement {
        if num >= prime || num < 0 {
            panic!("Num {} not in field range 0 to {}", num, prime - 1);
        }
        FieldElement { num, prime }
    }
    fn pow(&self, exponent: i32) -> FieldElement {
        let n = exponent % (self.prime - 1);
        let num = self.num.pow(n as u32) % self.prime;
        FieldElement::new(num, self.prime)
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &FieldElement) -> bool {
        self.num == other.num && self.prime == other.prime
    }
    fn ne(&self, other: &FieldElement) -> bool {
        !self.eq(other)
    }
}

impl Add for FieldElement {
    type Output = FieldElement;

    fn add(self, other: FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different Fields");
        }
        let num = (self.num + other.num) % self.prime;
        FieldElement::new(num, self.prime)
    }
}

impl Sub for FieldElement {
    type Output = FieldElement;

    fn sub(self, other: FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot subtract two numbers in different Fields");
        }
        let num = self.num - other.num;
        let num = if num < 0 {
            num % self.prime + self.prime
        } else {
            num % self.prime
        };
        FieldElement::new(num, self.prime)
    }
}

impl Mul for FieldElement {
    type Output = FieldElement;

    fn mul(self, other: FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot multiply two numbers in different Fields");
        }
        let num = (self.num * other.num) % self.prime;
        FieldElement::new(num, self.prime)
    }
}

impl Div for FieldElement {
    type Output = FieldElement;

    fn div(self, other: FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot divide two numbers in different Fields");
        }
        let other_inverse =
            FieldElement::new(mod_pow(other.num, self.prime - 2, self.prime), self.prime);
        self * other_inverse
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ne() {
        let a = FieldElement::new(2, 31);
        let b = FieldElement::new(2, 31);
        let c = FieldElement::new(15, 31);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_add() {
        let a = FieldElement::new(2, 31);
        let b = FieldElement::new(15, 31);
        let c = FieldElement::new(17, 31);
        assert_eq!(a + b, c);
        let a = FieldElement::new(17, 31);
        let b = FieldElement::new(21, 31);
        let c = FieldElement::new(7, 31);
        assert_eq!(a + b, c);
    }

    #[test]
    fn test_sub() {
        let a = FieldElement::new(29, 31);
        let b = FieldElement::new(4, 31);
        let c = FieldElement::new(25, 31);
        assert_eq!(a - b, c);
        let a = FieldElement::new(15, 31);
        let b = FieldElement::new(30, 31);
        let c = FieldElement::new(16, 31);
        assert_eq!(a - b, c);
    }

    #[test]
    fn test_mul() {
        let a = FieldElement::new(24, 31);
        let b = FieldElement::new(19, 31);
        let c = FieldElement::new(22, 31);
        assert_eq!(a * b, c);
    }

    #[test]
    fn test_div() {
        let a = FieldElement::new(3, 31);
        let b = FieldElement::new(24, 31);
        let c = FieldElement::new(4, 31);
        assert_eq!(a / b, c);
    }

    #[test]
    fn test_pow() {
        let a = FieldElement::new(17, 31);
        let b = FieldElement::new(15, 31);
        assert_eq!(a.pow(3), b);
        let a = FieldElement::new(5, 31);
        let b = FieldElement::new(18, 31);
        let c = FieldElement::new(16, 31);
        assert_eq!(a.pow(5) * b, c);
    }
}
