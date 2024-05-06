use crate::secp256k1::FieldElement;
use bnum::types::U512;
use std::ops::{Add, AddAssign, Mul};

static ORDER: U512 = U512::parse_str_radix(
    "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",
    16,
);

static A: FieldElement = FieldElement { num: U512::ZERO };

static B: FieldElement = FieldElement { num: U512::SEVEN };

#[allow(dead_code)]
static GENERATOR: Point = Point {
    x: Some(FieldElement {
        num: U512::parse_str_radix(
            "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
            16,
        ),
    }),
    y: Some(FieldElement {
        num: U512::parse_str_radix(
            "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8",
            16,
        ),
    }),
};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Point {
    x: Option<FieldElement>,
    y: Option<FieldElement>,
}

impl Point {
    pub fn new(x: Option<FieldElement>, y: Option<FieldElement>) -> Result<Self, String> {
        if x.is_none() && y.is_none() {
            return Ok(Self { x, y });
        }
        if x.is_none() || y.is_none() {
            return Err("Either both x and y must be None or neither".to_string());
        }
        let x_value = x.unwrap();
        let y_value = y.unwrap();
        if y_value.pow(U512::TWO, false) != x_value.pow(U512::THREE, false) + A * x_value + B {
            return Err(format!(
                "({:?}, {:?}) is not on the curve",
                x_value, y_value
            ));
        }
        Ok(Self { x, y })
    }

    #[allow(dead_code)]
    pub fn x(&self) -> Option<FieldElement> {
        self.x
    }

    #[allow(dead_code)]
    pub fn y(&self) -> Option<FieldElement> {
        self.y
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.x.is_none() {
            return rhs;
        }
        if rhs.x.is_none() {
            return self;
        }
        let x1 = self.x.unwrap();
        let y1 = self.y.unwrap();
        let x2 = rhs.x.unwrap();
        let y2 = rhs.y.unwrap();
        if x1 == x2 && y1 != y2 {
            return Self::new(None, None).unwrap();
        }
        let (x3, y3) = if x1 != x2 {
            let s = (y2 - y1) / (x2 - x1);
            let x3 = s.pow(U512::TWO, false) - x1 - x2;
            let y3 = s * (x1 - x3) - y1;
            (x3, y3)
        } else if self == rhs {
            if y1 == FieldElement::new(U512::ZERO) {
                return Self::new(None, None).unwrap();
            }
            let field_2 = FieldElement::new(U512::TWO);
            let field_3 = FieldElement::new(U512::THREE);
            let s = (field_3 * x1.pow(U512::TWO, false) + A) / (field_2 * y1);
            let x3 = s.pow(U512::TWO, false) - field_2 * x1;
            let y3 = s * (x1 - x3) - y1;
            (x3, y3)
        } else {
            panic!("Could not add points {:?}, {:?}", self, rhs);
        };
        Self::new(Some(x3), Some(y3)).unwrap()
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        *self = *self + rhs;
    }
}

impl Mul<U512> for Point {
    type Output = Self;

    fn mul(self, rhs: U512) -> Self::Output {
        let mut coef = rhs;
        coef %= ORDER;
        let mut current = self;
        let mut result = Self::new(None, None).unwrap();
        while coef > U512::ZERO {
            if coef & U512::ONE == U512::ONE {
                result += current;
            }
            current += current;
            coef >>= 1;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generator_order() {
        println!("{:?}", GENERATOR * ORDER);
        assert_eq!(GENERATOR * ORDER, Point::new(None, None).unwrap());
    }
}
