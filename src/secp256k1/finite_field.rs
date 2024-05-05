use bnum::types::I512;
use once_cell::sync::Lazy;
use std::ops::{Add, Div, Mul, Sub};

static PRIME: Lazy<I512> = Lazy::new(|| {
    I512::from_str_radix(
        "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        16,
    )
    .unwrap()
});

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct FieldElement {
    num: I512,
}

impl FieldElement {
    pub fn new(num: I512) -> Result<Self, String> {
        if num >= *PRIME || num < I512::from(0) {
            return Err(format!(
                "Num {num} not in field range 0 to {}",
                *PRIME - I512::from(1)
            )
            .to_string());
        }
        Ok(Self { num })
    }

    pub fn pow(&self, mut exp: I512) -> Self {
        if exp < I512::from(0) {
            exp = *PRIME - I512::from(1) + exp % (*PRIME - I512::from(1));
        }
        let mut result = I512::from(1);
        let mut base = self.num % *PRIME;
        while exp > I512::from(0) {
            if exp % I512::from(2) == I512::from(1) {
                result = result * base % *PRIME;
            }
            exp = exp >> 1;
            base = base * base % *PRIME;
        }
        Self::new(result).unwrap()
    }

    #[allow(dead_code)]
    pub fn num(&self) -> I512 {
        self.num
    }
}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let num = (self.num + rhs.num) % *PRIME;
        Self::new(num).unwrap()
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let num = self.num - rhs.num;
        let num = if num < I512::from(0) {
            num % *PRIME + *PRIME
        } else {
            num % *PRIME
        };
        Self::new(num).unwrap()
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let num = (self.num * rhs.num) % *PRIME;
        Self::new(num).unwrap()
    }
}

impl Div for FieldElement {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let other_inverse = rhs.pow(*PRIME - I512::from(2));
        self * other_inverse
    }
}
