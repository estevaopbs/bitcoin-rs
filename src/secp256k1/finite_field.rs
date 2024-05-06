use bnum::types::U512;
use std::ops::{Add, Div, Mul, Sub};

const PRIME: U512 = U512::parse_str_radix(
    "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
    16,
);

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct FieldElement {
    pub(crate) num: U512,
}

impl FieldElement {
    pub fn new(num: U512) -> Self {
        Self { num: num % PRIME }
    }

    pub fn pow(&self, mut exp: U512, is_negative: bool) -> Self {
        if is_negative {
            exp = PRIME - U512::ONE - exp % (PRIME - U512::ONE);
        }
        let mut result = U512::ONE;
        let mut base = self.num % PRIME;
        while exp > U512::ZERO {
            if exp % U512::TWO == U512::ONE {
                result *= base % PRIME;
            }
            exp >>= 1;
            base = base * base % PRIME;
        }
        Self::new(result)
    }

    #[allow(dead_code)]
    pub fn num(&self) -> U512 {
        self.num
    }
}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let num = (self.num + rhs.num) % PRIME;
        Self::new(num)
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let num = self.num - rhs.num;
        let num = if num < U512::ZERO {
            num % PRIME + PRIME
        } else {
            num % PRIME
        };
        Self::new(num)
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let num = (self.num * rhs.num) % PRIME;
        Self::new(num)
    }
}

impl Div for FieldElement {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let other_inverse = rhs.pow(PRIME - U512::TWO, false);
        self * other_inverse
    }
}
