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
