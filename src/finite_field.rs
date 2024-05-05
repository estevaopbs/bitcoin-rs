use bnum::types::I512;
use std::ops::{Add, Div, Mul, Sub};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct FieldElement {
    num: I512,
    prime: I512,
}

impl FieldElement {
    pub fn new(num: I512, prime: I512) -> Result<Self, String> {
        if num >= prime || num < I512::from(0) {
            return Err(format!(
                "Num {num} not in field range 0 to {}",
                prime - I512::from(1)
            )
            .to_string());
        }
        Ok(Self { num, prime })
    }

    pub fn pow(&self, mut exp: I512) -> Self {
        if self.prime == I512::from(1) {
            return Self::new(I512::from(0), I512::from(1)).unwrap();
        }
        if exp < I512::from(0) {
            exp = self.prime - I512::from(1) + exp % (self.prime - I512::from(1));
        }
        let mut result = I512::from(1);
        let mut base = self.num % self.prime;
        while exp > I512::from(0) {
            if exp % I512::from(2) == I512::from(1) {
                result = result * base % self.prime;
            }
            exp = exp >> 1;
            base = base * base % self.prime;
        }
        Self::new(result, self.prime).unwrap()
    }

    #[allow(dead_code)]
    pub fn num(&self) -> I512 {
        self.num
    }

    pub fn prime(&self) -> I512 {
        self.prime
    }
}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("Cannot add two numbers in different Fields");
        }
        let num = (self.num + rhs.num) % self.prime;
        Self::new(num, self.prime).unwrap()
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("Cannot subtract two numbers in different Fields");
        }
        let num = self.num - rhs.num;
        let num = if num < I512::from(0) {
            num % self.prime + self.prime
        } else {
            num % self.prime
        };
        Self::new(num, self.prime).unwrap()
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("Cannot multiply two numbers in different Fields");
        }
        let num = (self.num * rhs.num) % self.prime;
        Self::new(num, self.prime).unwrap()
    }
}

impl Div for FieldElement {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("Cannot divide two numbers in different Fields");
        }
        let other_inverse = rhs.pow(self.prime - I512::from(2));
        self * other_inverse
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ne() {
        let a = FieldElement::new(I512::from(2), I512::from(31)).unwrap();
        let b = FieldElement::new(I512::from(2), I512::from(31)).unwrap();
        let c = FieldElement::new(I512::from(15), I512::from(31)).unwrap();
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_add() {
        let a = FieldElement::new(I512::from(2), I512::from(31)).unwrap();
        let b = FieldElement::new(I512::from(15), I512::from(31)).unwrap();
        let c = FieldElement::new(I512::from(17), I512::from(31)).unwrap();
        assert_eq!(a + b, c);
        let a = FieldElement::new(I512::from(17), I512::from(31)).unwrap();
        let b = FieldElement::new(I512::from(21), I512::from(31)).unwrap();
        let c = FieldElement::new(I512::from(7), I512::from(31)).unwrap();
        assert_eq!(a + b, c);
    }

    #[test]
    fn test_sub() {
        let a = FieldElement::new(I512::from(29), I512::from(31)).unwrap();
        let b = FieldElement::new(I512::from(4), I512::from(31)).unwrap();
        let c = FieldElement::new(I512::from(25), I512::from(31)).unwrap();
        assert_eq!(a - b, c);
        let a = FieldElement::new(I512::from(15), I512::from(31)).unwrap();
        let b = FieldElement::new(I512::from(30), I512::from(31)).unwrap();
        let c = FieldElement::new(I512::from(16), I512::from(31)).unwrap();
        assert_eq!(a - b, c);
    }

    #[test]
    fn test_mul() {
        let a = FieldElement::new(I512::from(24), I512::from(31)).unwrap();
        let b = FieldElement::new(I512::from(19), I512::from(31)).unwrap();
        let c = FieldElement::new(I512::from(22), I512::from(31)).unwrap();
        assert_eq!(a * b, c);
    }

    #[test]
    fn test_div() {
        let a = FieldElement::new(I512::from(3), I512::from(31)).unwrap();
        let b = FieldElement::new(I512::from(24), I512::from(31)).unwrap();
        let c = FieldElement::new(I512::from(4), I512::from(31)).unwrap();
        assert_eq!(a / b, c);
    }

    #[test]
    fn test_pow() {
        let a = FieldElement::new(I512::from(17), I512::from(31)).unwrap();
        let b = FieldElement::new(I512::from(15), I512::from(31)).unwrap();
        assert_eq!(a.pow(I512::from(3)), b);
        let a = FieldElement::new(I512::from(5), I512::from(31)).unwrap();
        let b = FieldElement::new(I512::from(18), I512::from(31)).unwrap();
        let c = FieldElement::new(I512::from(16), I512::from(31)).unwrap();
        assert_eq!(a.pow(I512::from(5)) * b, c);
        let a = FieldElement::new(I512::from(7), I512::from(13)).unwrap();
        let b = FieldElement::new(I512::from(8), I512::from(13)).unwrap();
        assert_eq!(a.pow(I512::from(-3)), b);
    }
}
