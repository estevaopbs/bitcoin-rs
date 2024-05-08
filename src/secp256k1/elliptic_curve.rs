use crate::helper::{mod_pow, u256_to_u512, u512_to_u256};
use crate::secp256k1::{FieldElement, Signature};
use bnum::types::U256;
use std::ops::{Add, AddAssign, Mul};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Point {
    x: Option<FieldElement>,
    y: Option<FieldElement>,
}

impl Point {
    pub const B: FieldElement = FieldElement { num: U256::SEVEN };

    #[allow(dead_code)]
    pub const G: Point = Point {
        x: Some(FieldElement {
            num: U256::parse_str_radix(
                "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
                16,
            ),
        }),
        y: Some(FieldElement {
            num: U256::parse_str_radix(
                "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8",
                16,
            ),
        }),
    };

    pub const N: U256 = U256::parse_str_radix(
        "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",
        16,
    );

    pub const INFINITY: Point = Point { x: None, y: None };

    pub fn new(x: Option<U256>, y: Option<U256>) -> Result<Self, String> {
        let x = x.map(FieldElement::new);
        let y = y.map(FieldElement::new);
        Self::from_field_elements(x, y)
    }

    pub fn from_field_elements(
        x: Option<FieldElement>,
        y: Option<FieldElement>,
    ) -> Result<Self, String> {
        if x.is_none() && y.is_none() {
            return Ok(Self::INFINITY);
        }
        if x.is_none() || y.is_none() {
            return Err("Either both x and y must be None or neither".to_string());
        }
        let x = x.unwrap();
        let y = y.unwrap();
        if y.pow(U256::TWO, false) != x.pow(U256::THREE, false) + Self::B {
            return Err(format!("({:?}, {:?}) is not on the curve", x, y));
        }
        Ok(Self {
            x: Some(x),
            y: Some(y),
        })
    }

    #[allow(dead_code)]
    pub fn x(&self) -> Option<FieldElement> {
        self.x
    }

    #[allow(dead_code)]
    pub fn y(&self) -> Option<FieldElement> {
        self.y
    }

    pub fn is_infinity(&self) -> bool {
        self.x.is_none()
    }

    #[allow(dead_code)]
    pub fn verify(&self, z: U256, sig: Signature) -> bool {
        let s_inv = mod_pow(sig.s().num(), Point::N - U256::TWO, false, Point::N);
        let u = u512_to_u256(u256_to_u512(z) * u256_to_u512(s_inv) % u256_to_u512(Point::N));
        let v = u512_to_u256(
            u256_to_u512(sig.r().num()) * u256_to_u512(s_inv) % u256_to_u512(Point::N),
        );
        let total = Point::G * u + *self * v;
        total.x.unwrap() == sig.r()
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.is_infinity() {
            return rhs;
        }
        if rhs.is_infinity() {
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
            let x3 = s.pow(U256::TWO, false) - x1 - x2;
            let y3 = s * (x1 - x3) - y1;
            (x3, y3)
        } else if self == rhs {
            if y1 == FieldElement::new(U256::ZERO) {
                return Self::new(None, None).unwrap();
            }
            let field_2 = FieldElement::new(U256::TWO);
            let field_3 = FieldElement::new(U256::THREE);
            let s = (field_3 * x1.pow(U256::TWO, false)) / (field_2 * y1);
            let x3 = s.pow(U256::TWO, false) - field_2 * x1;
            let y3 = s * (x1 - x3) - y1;
            (x3, y3)
        } else {
            panic!("Could not add points {:?}, {:?}", self, rhs);
        };
        Self::from_field_elements(Some(x3), Some(y3)).unwrap()
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        *self = *self + rhs;
    }
}

impl Mul<U256> for Point {
    type Output = Self;

    fn mul(self, rhs: U256) -> Self::Output {
        let mut coef = rhs;
        coef %= Self::N;
        let mut current = self;
        let mut result = Self::new(None, None).unwrap();
        while coef > U256::ZERO {
            if coef & U256::ONE == U256::ONE {
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
        assert_eq!(Point::G * Point::N, Point::INFINITY);
    }

    #[test]
    fn test_verify() {
        let z = U256::parse_str_radix(
            "BC62D4B80D9E36DA29C16C5D4D9F11731F36052C72401A76C23C0FB5A9B74423",
            16,
        );
        let r = U256::parse_str_radix(
            "37206A0610995C58074999CB9767B87AF4C4978DB68C06E8E6E81D282047A7C6",
            16,
        );
        let s = U256::parse_str_radix(
            "8CA63759C1157EBEAEC0D03CECCA119FC9A75BF8E6D0FA65C841C8E2738CDAEC",
            16,
        );
        let px = U256::parse_str_radix(
            "04519FAC3D910CA7E7138F7013706F619FA8F033E6EC6E09370EA38CEE6A7574",
            16,
        );
        let py = U256::parse_str_radix(
            "82B51EAB8C27C66E26C858A079BCDF4F1ADA34CEC420CAFC7EAC1A42216FB6C4",
            16,
        );
        let point = Point::new(Some(px), Some(py)).unwrap();
        let sig = Signature::new(r, s);
        assert!(point.verify(z, sig));
    }
}
