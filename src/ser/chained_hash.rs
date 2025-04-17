use hmac::digest::Digest;
use std::marker::PhantomData;

pub trait ChainedCompute<const R: usize, H, F>
where
    H: Digest,
    F: Digest,
{
    fn compute(data: &[u8]) -> Vec<u8> {
        let mut hasher = H::new();
        hasher.update(data);
        let mut result = hasher.finalize().to_vec();
        for _ in 1..R {
            let mut hasher = H::new();
            hasher.update(&result);
            result = hasher.finalize().to_vec();
        }
        let mut finalizer = F::new();
        finalizer.update(&result);
        finalizer.finalize().to_vec()
    }
}

pub struct ChainedHasher<const R: usize, H, F>
where
    H: Digest,
    F: Digest,
{
    _marker_h: PhantomData<H>,
    _marker_f: PhantomData<F>,
}

impl<const R: usize, H, F> ChainedCompute<R, H, F> for ChainedHasher<R, H, F>
where
    H: Digest,
    F: Digest,
{
}

macro_rules! chained_hash {
    ($name: ident, $hasher:ty, $finalizer: ty, $rounds: expr) => {
        pub type $name = crate::ser::chained_hash::ChainedHasher<$rounds, $hasher, $finalizer>;
    };
}
