use bnum::types::{U256, U512};
use bnum::BTryFrom;
use digest::generic_array::GenericArray;
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

#[derive(Debug)]
pub struct S256PrivateKey {
    secret: S256Field,
    point: S256Point,
}

impl S256PrivateKey {
    pub fn new(secret: S256Field) -> Self {
        Self {
            secret,
            point: *S256Point::G * secret.num(),
        }
    }

    pub fn secret(&self) -> S256Field {
        self.secret
    }

    pub fn point(&self) -> S256Point {
        self.point
    }

    pub fn sign(&self, z: U256) -> S256Signature {
        let k = self.deterministic_k(z);
        let r = (*S256Point::G * k).x().unwrap().num();
        let k_inv = S256Field::mod_pow(k, *S256Point::N - U256::TWO, false, *S256Point::N);
        let mut s = (z + r * self.secret.num()) * k_inv % *S256Point::N;
        if s > *S256Point::N / U256::TWO {
            s = *S256Point::N - s;
        }
        S256Signature::from_values(r, s)
    }

    fn deterministic_k(&self, mut z: U256) -> U256 {
        let k = b"\x00".repeat(32);
        let v = b"\x01".repeat(32);
        if z > *S256Point::N {
            z -= *S256Point::N;
        }
        let z_bytes = z.to_be_bytes();
        let secret_bytes = self.secret.num().to_be_bytes();
        let mut k_ = Hmac::<Sha256>::new(GenericArray::from_slice(&k));
        k_.update(&[&v[..], &[0u8], &secret_bytes[..], &z_bytes[..]].concat());
        let mut k = k_.finalize().into_bytes();
        let mut v_ = Hmac::<Sha256>::new(GenericArray::from_slice(&k));
        v_.update(&v[..]);
        let mut v = v_.finalize().into_bytes();
        k_ = Hmac::<Sha256>::new(GenericArray::from_slice(&k));
        k_.update(&[&v[..], &[0u8], &secret_bytes[..], &z_bytes[..]].concat());
        k = k_.finalize().into_bytes();
        v_ = Hmac::<Sha256>::new(GenericArray::from_slice(&k));
        v_.update(&v[..]);
        v = v_.finalize().into_bytes();
        loop {
            v_ = Hmac::<Sha256>::new(GenericArray::from_slice(&k));
            v_.update(&v[..]);
            v = v_.finalize().into_bytes();
            let candidate = U256::from_be_bytes(v.try_into().unwrap());
            if candidate >= U256::ONE && candidate < *S256Point::N {
                return candidate;
            }
            k_ = Hmac::<Sha256>::new(GenericArray::from_slice(&k));
            k_.update(&[&v[..], &[0u8]].concat());
            k = k_.finalize().into_bytes();
            v_ = Hmac::<Sha256>::new(GenericArray::from_slice(&k));
            v_.update(&v[..]);
            v = v_.finalize().into_bytes();
        }
    }
}
