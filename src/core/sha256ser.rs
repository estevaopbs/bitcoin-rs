use bnum::types::U512;
use ripemd::{Digest, Ripemd160};
use sha2::Sha256;

encode_base58!(encode_base58, U512, 64);

hash!(hash256, Sha256, 2);

encode_base58_checksum!(encode_base58_checksum, hash256, encode_base58);

hash160!(hash160, Sha256);
