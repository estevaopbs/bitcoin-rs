use super::chained_hash::ChainedCompute;
use bnum::BUint;
use hmac::digest::Digest;
use std::convert::TryInto;
use std::marker::PhantomData;

pub trait Base58<C, const R: usize, H, F, const N: usize>
where
    H: Digest,
    F: Digest,
    C: ChainedCompute<R, H, F>,
    [(); BUint::<N>::BYTES_USIZE]:,
{
    const BASE58_ALPHABET: &'static str;

    fn encode_base58(s: &[u8]) -> String {
        let num_58 = BUint::<N>::from_digit(58);
        let zeros = s.iter().take_while(|&&c| c == 0).count();
        let prefix = "1".repeat(zeros);

        let mut extended_s = vec![0; BUint::<N>::BYTES_USIZE - s.len()];
        extended_s.extend_from_slice(&s);

        let mut num = BUint::<N>::from_be_bytes(extended_s.try_into().unwrap());

        let mut result = String::new();
        while num > BUint::<N>::ZERO {
            let rem: usize = (num % num_58).try_into().unwrap();
            result.push(Self::BASE58_ALPHABET.chars().nth(rem).unwrap());
            num /= num_58;
        }
        result.push_str(&prefix);
        result.chars().rev().collect()
    }

    fn encode_base58_with_checksum(mut data: Vec<u8>) -> String {
        let checksum = C::compute(&data);
        data.extend_from_slice(&checksum[..4]);
        Self::encode_base58(&data)
    }
}

pub struct Base58ChainedHasher<C: ChainedCompute<R, H, F>, const R: usize, H, F, const N: usize>
where
    H: Digest,
    F: Digest,
    C: ChainedCompute<R, H, F>,
    [(); BUint::<N>::BYTES_USIZE]:,
{
    _marker_c: PhantomData<C>,
    _marker_h: PhantomData<H>,
    _marker_f: PhantomData<F>,
}

impl<C, const R: usize, H, F, const N: usize> Base58<C, R, H, F, N>
    for Base58ChainedHasher<C, R, H, F, N>
where
    C: ChainedCompute<R, H, F>,
    [(); BUint::<N>::BYTES_USIZE]:,
    H: Digest,
    F: Digest,
{
    const BASE58_ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
}

macro_rules! base58 {
    ($name:ident, $chained_hasher: ident, $rounds: expr, $hasher: ident, $finalizer: ident, $bits: expr) => {
        pub type $name = crate::ser::base58::Base58ChainedHasher<
            $chained_hasher,
            $rounds,
            $hasher,
            $finalizer,
            { $bits / 64 },
        >;
    };
}
