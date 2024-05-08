use crate::helper::mod_pow;
use crate::secp256k1::{FieldElement, Point, Signature};
use bnum::types::U256;
use digest::generic_array::GenericArray;
use hmac::{Hmac, Mac};
use sha2::Sha256;

#[derive(Debug)]
pub struct PrivateKey {
    secret: FieldElement,
    #[allow(dead_code)]
    point: Point,
}

impl PrivateKey {
    #[allow(dead_code)]
    pub fn new(secret: FieldElement, point: Point) -> Self {
        Self { secret, point }
    }

    #[allow(dead_code)]
    pub fn secret(&self) -> FieldElement {
        self.secret
    }

    #[allow(dead_code)]
    pub fn sign(&self, z: U256) -> Signature {
        let k = self.deterministic_k(z);
        let r = (Point::G * k).x().unwrap().num();
        let k_inv = mod_pow(k, Point::N - U256::TWO, false, Point::N);
        let mut s = (z + r * self.secret.num()) * k_inv % Point::N;
        if s > Point::N / U256::TWO {
            s = Point::N - s;
        }
        Signature::new(r, s)
    }

    #[allow(dead_code)]
    fn deterministic_k(&self, mut z: U256) -> U256 {
        let k = b"\x00".repeat(32);
        let v = b"\x01".repeat(32);
        if z > Point::N {
            z -= Point::N;
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
            if candidate >= U256::ONE && candidate < Point::N {
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
