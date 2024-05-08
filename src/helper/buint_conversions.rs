use bnum::types::{U256, U512};
use bnum::BTryFrom;

pub fn u512_to_u256(num: U512) -> U256 {
    <U256 as BTryFrom<U512>>::try_from(num).unwrap()
}

pub fn u256_to_u512(num: U256) -> U512 {
    <U512 as BTryFrom<U256>>::try_from(num).unwrap()
}
