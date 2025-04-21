use super::elliptic_curve::{EllipticCurve, Point};
use super::finite_field::{FieldElement, Modulus};
use super::signature::Signature;
use bnum::cast::As;
use bnum::BUint;
use hmac::digest::block_buffer::Eager;
use hmac::digest::consts::U256;
use hmac::digest::core_api::{
    BlockSizeUser, BufferKindUser, CoreProxy, FixedOutputCore, UpdateCore,
};
use hmac::digest::typenum::{IsLess, Le, NonZero};
use hmac::digest::HashMarker;
use hmac::{Hmac, Mac};
use std::marker::PhantomData;

pub struct PrivateKey<E: EllipticCurve<M, N>, M: Modulus<N>, const N: usize, H>
where
    H: CoreProxy,
    H::Core: HashMarker
        + UpdateCore
        + FixedOutputCore
        + BufferKindUser<BufferKind = Eager>
        + Default
        + Clone,
    <H::Core as BlockSizeUser>::BlockSize: IsLess<U256>,
    Le<<H::Core as BlockSizeUser>::BlockSize, U256>: NonZero,
    [(); 2 * N]:,
{
    secret: FieldElement<M, N>,
    point: Point<E, M, N>,
    _marker: PhantomData<H>,
}

impl<E: EllipticCurve<M, N>, M: Modulus<N>, const N: usize, H> PrivateKey<E, M, N, H>
where
    H: CoreProxy,
    H::Core: HashMarker
        + UpdateCore
        + FixedOutputCore
        + BufferKindUser<BufferKind = Eager>
        + Default
        + Clone,
    <H::Core as BlockSizeUser>::BlockSize: IsLess<U256>,
    Le<<H::Core as BlockSizeUser>::BlockSize, U256>: NonZero,
    [(); 2 * N]:,
    [(); N / 8]:,
    [(); BUint::<N>::BYTES_USIZE]:,
    [(); 4 * N]:,
{
    #[inline]
    pub fn new(secret: FieldElement<M, N>) -> Self {
        Self {
            secret,
            point: *Point::<E, M, N>::G * secret.num(),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn from_value(secret: BUint<N>) -> Self {
        Self::new(FieldElement::<M, N>::new(secret))
    }

    #[inline]
    pub fn secret(&self) -> FieldElement<M, N> {
        self.secret
    }

    #[inline]
    pub fn point(&self) -> Point<E, M, N> {
        self.point
    }

    #[inline]
    fn to_big(x: BUint<N>) -> BUint<{ 4 * N }> {
        x.as_()
    }

    #[inline]
    fn from_big(x: BUint<{ 4 * N }>) -> BUint<N> {
        x.as_()
    }

    #[inline]
    pub fn sign(&self, z: BUint<N>) -> Signature<M, N> {
        let k = self.deterministic_k(z);
        let r = (*Point::<E, M, N>::G * k).x().unwrap().num();
        let k_inv = FieldElement::<M, N>::mod_pow(k, E::N - BUint::<N>::TWO, false, E::N);
        let mut s = Self::from_big(
            (Self::to_big(z) + Self::to_big(r) * Self::to_big(self.secret.num()))
                * Self::to_big(k_inv)
                % Self::to_big(E::N),
        );
        if s > E::N / BUint::<N>::TWO {
            s = E::N - s;
        }
        Signature::<M, N>::from_values(r, s)
    }

    #[inline]
    fn deterministic_k(&self, mut z: BUint<N>) -> BUint<N> {
        let k = [0u8; BUint::<N>::BYTES_USIZE];
        let v = [1u8; BUint::<N>::BYTES_USIZE];
        if z > E::N {
            z -= E::N;
        }
        let z_bytes = z.to_be_bytes();
        let secret_bytes = self.secret.num().to_be_bytes();
        let mut k = Hmac::<H>::new_from_slice(&k)
            .unwrap()
            .chain_update(&[&v[..], &[0u8], &secret_bytes[..], &z_bytes[..]].concat())
            .finalize()
            .into_bytes();
        let mut v = Hmac::<H>::new_from_slice(&k)
            .unwrap()
            .chain_update(&v[..])
            .finalize()
            .into_bytes();
        k = Hmac::<H>::new_from_slice(&k)
            .unwrap()
            .chain_update(&[&v[..], &[1u8]].concat())
            .finalize()
            .into_bytes();
        v = Hmac::<H>::new_from_slice(&k)
            .unwrap()
            .chain_update(&v[..])
            .finalize()
            .into_bytes();
        loop {
            v = Hmac::<H>::new_from_slice(&k)
                .unwrap()
                .chain_update(&v[..])
                .finalize()
                .into_bytes();
            let mut bytes = [0u8; BUint::<N>::BYTES_USIZE];
            let hash_bytes = v.as_slice();

            let len = core::cmp::min(bytes.len(), hash_bytes.len());
            bytes[..len].copy_from_slice(&hash_bytes[..len]);

            let candidate = BUint::<N>::from_be_bytes(bytes);

            if candidate >= BUint::<N>::ONE && candidate < E::N {
                return candidate;
            }
            k = Hmac::<H>::new_from_slice(&k)
                .unwrap()
                .chain_update(&[&v[..], &[0u8]].concat())
                .finalize()
                .into_bytes();
            v = Hmac::<H>::new_from_slice(&k)
                .unwrap()
                .chain_update(&v[..])
                .finalize()
                .into_bytes();
        }
    }
}

macro_rules! private_key {
    ($name: ident, $curve_config: ident ,$modulus: ident, $bits: expr, $hasher: ty) => {
        pub type $name =
            crate::ecc::private_key::PrivateKey<$curve_config, $modulus, { $bits / 64 }, $hasher>;
    };
}
