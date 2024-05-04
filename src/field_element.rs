use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug)]
pub struct FieldElement {
    num: i32,
    prime: i32,
}

impl FieldElement {
    pub fn new(num: i32, prime: i32) -> Self {
        if num >= prime || num < 0 {
            panic!("Num {} not in field range 0 to {}", num, prime - 1);
        }
        Self { num, prime }
    }

    pub fn pow(&self, mut exp: i32) -> Self {
        if self.prime == 1 {
            return Self::new(0, 1);
        }
        if exp < 0 {
            exp = self.prime - 1 + exp % (self.prime - 1);
        }
        let mut result = 1;
        let mut base = self.num % self.prime;
        while exp > 0 {
            if exp % 2 == 1 {
                result = result * base % self.prime;
            }
            exp = exp >> 1;
            base = base * base % self.prime;
        }
        Self::new(result, self.prime)
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.prime == other.prime
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different Fields");
        }
        let num = (self.num + other.num) % self.prime;
        Self::new(num, self.prime)
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            panic!("Cannot subtract two numbers in different Fields");
        }
        let num = self.num - other.num;
        let num = if num < 0 {
            num % self.prime + self.prime
        } else {
            num % self.prime
        };
        Self::new(num, self.prime)
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            panic!("Cannot multiply two numbers in different Fields");
        }
        let num = (self.num * other.num) % self.prime;
        Self::new(num, self.prime)
    }
}

impl Div for FieldElement {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            panic!("Cannot divide two numbers in different Fields");
        }
        let other_inverse = other.pow(self.prime - 2);
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
        let a = FieldElement::new(7, 13);
        let b = FieldElement::new(8, 13);
        assert_eq!(a.pow(-3), b);
    }
}
