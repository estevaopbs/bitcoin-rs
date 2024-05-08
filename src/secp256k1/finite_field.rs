use crate::helper::{mod_pow, u256_to_u512, u512_to_u256};
use bnum::types::{U256, U512};
use std::ops::{Add, Div, Mul, Sub};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct FieldElement {
    pub(crate) num: U256,
}

impl FieldElement {
    const PRIME: U256 = U256::parse_str_radix(
        "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        16,
    );

    const PRIME_512: U512 = U512::parse_str_radix(
        "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        16,
    );

    pub fn new(num: U256) -> Self {
        Self {
            num: num % Self::PRIME,
        }
    }

    pub fn pow(&self, exp: U256, is_negative: bool) -> Self {
        Self::new(mod_pow(self.num, exp, is_negative, Self::PRIME))
    }

    pub fn num(&self) -> U256 {
        self.num
    }

    fn num_512(&self) -> U512 {
        u256_to_u512(self.num)
    }
}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let num = (self.num_512() + u256_to_u512(rhs.num)) % Self::PRIME_512;
        Self::new(u512_to_u256(num))
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let num = if self.num < rhs.num {
            self.num + (Self::PRIME - rhs.num)
        } else {
            self.num - rhs.num
        };
        Self::new(num)
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let num = (self.num_512() * u256_to_u512(rhs.num)) % Self::PRIME_512;
        Self::new(u512_to_u256(num))
    }
}

impl Div for FieldElement {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let other_inverse = rhs.pow(Self::PRIME - U256::TWO, false);
        self * other_inverse
    }
}
