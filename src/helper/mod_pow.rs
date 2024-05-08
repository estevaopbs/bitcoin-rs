use crate::helper::{u256_to_u512, u512_to_u256};
use bnum::types::{U256, U512};

pub fn mod_pow(base: U256, exp: U256, is_negative: bool, modulus: U256) -> U256 {
    let mut result = U512::ONE;
    let mut base = u256_to_u512(base);
    let mut exp = exp;
    let modulus = u256_to_u512(modulus);
    while exp > U256::ZERO {
        if exp % U256::TWO == U256::ONE {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exp >>= 1;
    }
    if is_negative {
        result = modulus - result;
    }
    u512_to_u256(result)
}
