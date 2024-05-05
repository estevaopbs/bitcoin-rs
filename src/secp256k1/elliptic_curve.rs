use crate::secp256k1::finite_field::FieldElement;
use bnum::types::I512;
use once_cell::sync::Lazy;
use std::ops::{Add, AddAssign, Mul};

static ORDER: Lazy<I512> = Lazy::new(|| {
    I512::from_str_radix(
        "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",
        16,
    )
    .unwrap()
});

static A: Lazy<FieldElement> = Lazy::new(|| FieldElement::new(I512::from(0)).unwrap());

static B: Lazy<FieldElement> = Lazy::new(|| FieldElement::new(I512::from(7)).unwrap());

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
        if y_value.pow(I512::from(2)) != x_value.pow(I512::from(3)) + *A * x_value + *B {
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
            let x3 = s.pow(I512::from(2)) - x1 - x2;
            let y3 = s * (x1 - x3) - y1;
            (x3, y3)
        } else if self == rhs {
            if y1 == FieldElement::new(I512::from(0)).unwrap() {
                return Self::new(None, None).unwrap();
            }
            let field_2 = FieldElement::new(I512::from(2)).unwrap();
            let field_3 = FieldElement::new(I512::from(3)).unwrap();
            let s = (field_3 * x1.pow(I512::from(2)) + *A) / (field_2 * y1);
            let x3 = s.pow(I512::from(2)) - field_2 * x1;
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

impl Mul<I512> for Point {
    type Output = Self;

    fn mul(self, rhs: I512) -> Self::Output {
        let mut coef = rhs;
        coef %= *ORDER;
        let mut current = self;
        let mut result = Self::new(None, None).unwrap();
        while coef > I512::from(0) {
            if coef & I512::from(1) == I512::from(1) {
                result += current;
            }
            current += current;
            coef >>= 1;
        }
        result
    }
}
