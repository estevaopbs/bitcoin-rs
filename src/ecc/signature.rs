use super::finite_field::{FieldElement, Modulus};
use bnum::BUint;

pub struct Signature<M: Modulus<N>, const N: usize>
where
    [(); 2 * N]:,
{
    r: FieldElement<M, N>,
    s: FieldElement<M, N>,
}

impl<M: Modulus<N>, const N: usize> Signature<M, N>
where
    [(); 2 * N]:,
{
    pub const fn new(r: FieldElement<M, N>, s: FieldElement<M, N>) -> Self {
        Self { r, s }
    }

    pub fn from_values(r: BUint<N>, s: BUint<N>) -> Self {
        Self::new(FieldElement::<M, N>::new(r), FieldElement::<M, N>::new(s))
    }

    pub fn r(&self) -> FieldElement<M, N> {
        self.r
    }

    pub fn s(&self) -> FieldElement<M, N> {
        self.s
    }

    pub fn der(&self) -> Vec<u8>
    where
        [(); BUint::<N>::BYTES_USIZE]:,
    {
        let rbin = self.r.num().to_be_bytes().to_vec();
        let sbin = self.s.num().to_be_bytes().to_vec();
        let mut result = Vec::<u8>::new();
        let bins = [rbin, sbin];
        for mut bin in bins {
            bin = bin
                .iter()
                .skip_while(|&&x| x == 0)
                .cloned()
                .collect::<Vec<u8>>();
            if bin[0] & 0x80 != 0 {
                bin.splice(0..0, [0u8].iter().cloned());
            }
            result.push(2u8);
            result.push(bin.len() as u8);
            result.extend(bin);
        }
        result
    }
}

macro_rules! signature {
    ($name: ident, $modulus: ident, $bits: expr) => {
        pub type $name = crate::ecc::signature::Signature<$modulus, { $bits / 64 }>;
    };
}
