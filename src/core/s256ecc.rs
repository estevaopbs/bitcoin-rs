use crate::core::{encode_base58_checksum, hash160};
use bnum::types::{U1024, U256, U512};
use bnum::BTryFrom;
use hmac::{Hmac, Mac};
use once_cell::sync::Lazy;
use sha2::Sha256;
use std::ops::{Add, AddAssign, Div, Mul, Sub};

field_element!(
    S256Field,
    U256,
    U512,
    U256::parse_str_radix(
        "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        16,
    )
);

impl Sqrt for S256Field {
    fn sqrt(&self) -> Self {
        self.pow((*Self::PRIME + U256::ONE) / U256::FOUR, false)
    }
}

signature!(S256Signature, S256Field, U256);

point!(
    S256Point,
    S256Field,
    S256Signature,
    U256,
    U256::ZERO,
    U256::SEVEN,
    U256::parse_str_radix(
        "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
        16,
    ),
    U256::parse_str_radix(
        "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8",
        16,
    ),
    U256::parse_str_radix(
        "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",
        16,
    )
);

impl S256Point {
    pub fn hash160(&self, compressed: bool) -> Vec<u8> {
        hash160(self.sec(compressed))
    }

    pub fn address(&self, compressed: bool, testnet: bool) -> String {
        let h160 = self.hash160(compressed);
        let prefix = if testnet { vec![0x6f] } else { vec![0x00] };
        let mut payload = prefix;
        payload.extend_from_slice(&h160);
        encode_base58_checksum(payload)
    }
}

private_key!(
    S256PrivateKey,
    S256Field,
    S256Signature,
    S256Point,
    U256,
    U1024,
    32,
    Sha256
);

impl S256PrivateKey {
    pub fn wif(&self, compressed: bool, testnet: bool) -> String {
        let mut result = if testnet { vec![0xef] } else { vec![0x80] };
        result.extend_from_slice(&self.secret().num().to_be_bytes());
        if compressed {
            result.push(0x01);
        }
        encode_base58_checksum(result)
    }
}
