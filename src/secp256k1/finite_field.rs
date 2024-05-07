use bnum::types::{U256, U512};
use bnum::BTryFrom;
use std::ops::{Add, Div, Mul, Sub};

const PRIME: U256 = U256::parse_str_radix(
    "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
    16,
);

const PRIME_512: U512 = U512::parse_str_radix(
    "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
    16,
);

fn u512_to_u256(num: U512) -> U256 {
    <U256 as BTryFrom<U512>>::try_from(num).unwrap()
}

fn u256_to_u512(num: U256) -> U512 {
    <U512 as BTryFrom<U256>>::try_from(num).unwrap()
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct FieldElement {
    pub(crate) num: U256,
}

impl FieldElement {
    pub fn new(num: U256) -> Self {
        Self { num: num % PRIME }
    }

    pub fn pow(&self, mut exp: U256, is_negative: bool) -> Self {
        if is_negative {
            let mut exp_512 = u256_to_u512(self.num);
            exp_512 = PRIME_512 - U512::ONE - exp_512 % (PRIME_512 - U512::ONE);
            exp = u512_to_u256(exp_512);
        }
        let mut result = U256::ONE;
        let mut base = self.num % PRIME;
        while exp > U256::ZERO {
            if exp % U256::TWO == U256::ONE {
                result = result * base % PRIME;
            }
            exp >>= 1;
            base = base * base % PRIME;
        }
        Self::new(result)
    }

    #[allow(dead_code)]
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
        let num = (self.num_512() + u256_to_u512(rhs.num)) % PRIME_512;
        Self::new(u512_to_u256(num))
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let num = if self.num < rhs.num {
            self.num + (PRIME - rhs.num)
        } else {
            self.num - rhs.num
        };
        Self::new(num)
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let num = (self.num_512() * u256_to_u512(rhs.num)) % PRIME_512;
        Self::new(u512_to_u256(num))
    }
}

impl Div for FieldElement {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let other_inverse = rhs.pow(PRIME - U256::TWO, false);
        self * other_inverse
    }
}
