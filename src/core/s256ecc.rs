use super::sha256ser::*;
use crate::ecc::elliptic_curve::EllipticCurve;
use crate::ecc::finite_field::{FieldElement, Modulus, Sqrt};
use crate::ser::base58::Base58;
use crate::ser::chained_hash::ChainedCompute;
use bnum::BUint;
use once_cell::sync::Lazy;
use sha2::Sha256;

field_element!(
    S256Field,
    S256FieldCfg,
    256,
    "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
    16,
    p3mod4
);

signature!(S256Signature, S256FieldCfg, 256);

point!(
    S256Point,
    S256CurveCfg,
    S256FieldCfg,
    256,
    "0",
    "7",
    "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
    "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8",
    "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",
    16
);

impl S256Point {
    #[inline]
    pub fn hash160(&self, compressed: bool) -> Vec<u8> {
        Sha256Ripemd160::compute(self.sec(compressed).as_slice())
    }

    #[inline]
    pub fn address(&self, compressed: bool, testnet: bool) -> String {
        let h160 = self.hash160(compressed);
        let prefix = if testnet { vec![0x6f] } else { vec![0x00] };
        let mut payload = prefix;
        payload.extend_from_slice(&h160);
        Sha256Base58::encode_base58_with_checksum(payload)
    }
}

private_key!(S256PrivateKey, S256CurveCfg, S256FieldCfg, 256, Sha256);

impl S256PrivateKey {
    #[inline]
    pub fn wif(&self, compressed: bool, testnet: bool) -> String {
        let mut result = if testnet { vec![0xef] } else { vec![0x80] };
        result.extend_from_slice(&self.secret().num().to_be_bytes());
        if compressed {
            result.push(0x01);
        }
        Sha256Base58::encode_base58_with_checksum(result)
    }
}
