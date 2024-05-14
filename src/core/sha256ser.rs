use bnum::types::{U256, U512};
use ripemd::Digest as RipemdDigest;
use ripemd::Ripemd160;
use sha2::Digest as Sha256Digest;
use sha2::Sha256;

encode_base58!(encode_base58, U512, 64);

encode_base58_checksum!(encode_base58_checksum, U256, Sha256, encode_base58);

hash160!(hash160, Sha256);
