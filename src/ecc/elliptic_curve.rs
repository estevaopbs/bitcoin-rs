use super::finite_field::{FieldElement, Modulus, Sqrt};
use super::signature::Signature;
use bnum::BUint;
use once_cell::sync::Lazy;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Mul};

pub trait EllipticCurve<M: Modulus<N>, const N: usize>: PartialEq + Debug + Clone + Copy
where
    [(); 2 * N]:,
{
    const A: Lazy<FieldElement<M, N>>;
    const B: Lazy<FieldElement<M, N>>;
    const GX: Lazy<FieldElement<M, N>>;
    const GY: Lazy<FieldElement<M, N>>;
    const N: BUint<N>;
    const FIELD_0: FieldElement<M, N>;
    const FIELD_2: Lazy<FieldElement<M, N>>;
    const FIELD_3: Lazy<FieldElement<M, N>>;
    const SEC_X_END: usize;
    const SEC_Y_END: usize;
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Point<E, M, const N: usize>
where
    M: Modulus<N>,
    E: EllipticCurve<M, N>,
    [(); 2 * N]:,
{
    x: Option<FieldElement<M, N>>,
    y: Option<FieldElement<M, N>>,
    _marker: PhantomData<E>,
}

impl<E, M, const N: usize> Point<E, M, N>
where
    M: Modulus<N>,
    E: EllipticCurve<M, N>,
    [(); 2 * N]:,
{
    pub const G: Lazy<Self> = Lazy::new(|| Self::from_values(E::GX.num(), E::GY.num()).unwrap());

    pub const INFINITY: Point<E, M, N> = Point {
        x: None,
        y: None,
        _marker: PhantomData,
    };

    #[inline]
    fn is_on_curve(x: &FieldElement<M, N>, y: &FieldElement<M, N>) -> bool {
        y.pow(BUint::<N>::TWO, false) == x.pow(BUint::<N>::THREE, false) + *E::A * *x + *E::B
    }

    #[inline]
    pub fn new(
        x: Option<FieldElement<M, N>>,
        y: Option<FieldElement<M, N>>,
    ) -> Result<Self, String> {
        if x.is_none() && y.is_none() {
            return Ok(Self::INFINITY);
        }
        if x.is_none() || y.is_none() {
            return Err(
                "Invalid point: both x and y must be provided, or both must be None.".to_string(),
            );
        }
        let x = x.unwrap();
        let y = y.unwrap();
        if !Self::is_on_curve(&x, &y) {
            return Err(format!(
                "Invalid point: ({}, {}) does not satisfy the curve equation y^2 = x^3 + ax + b",
                x.num(),
                y.num()
            ));
        }
        Ok(Self {
            x: Some(x),
            y: Some(y),
            _marker: PhantomData,
        })
    }

    #[inline]
    pub fn from_values(x: BUint<N>, y: BUint<N>) -> Result<Self, String> {
        Self::new(
            Some(FieldElement::<M, N>::new(x)),
            Some(FieldElement::<M, N>::new(y)),
        )
    }

    #[inline]
    pub fn x(&self) -> Option<FieldElement<M, N>> {
        self.x
    }

    #[inline]
    pub fn y(&self) -> Option<FieldElement<M, N>> {
        self.y
    }

    #[inline]
    pub fn is_infinity(&self) -> bool {
        self.x.is_none()
    }

    #[inline]
    pub fn verify(&self, z: BUint<N>, sig: Signature<M, N>) -> bool {
        let s_inv =
            FieldElement::<M, N>::mod_pow(sig.s().num(), E::N - BUint::<N>::TWO, false, E::N);
        let u = M::from_big(M::to_big(z) * M::to_big(s_inv) % M::to_big(E::N));
        let v = M::from_big(M::to_big(sig.r().num()) * M::to_big(s_inv) % M::to_big(E::N));
        let total = *Self::G * u + *self * v;
        sig.r() == total.x.unwrap()
    }

    pub fn sec(&self, compressed: bool) -> Vec<u8>
    where
        [(); BUint::<N>::BYTES_USIZE]:,
    {
        if compressed {
            let first_byte = if self.y.unwrap().num() & BUint::<N>::ONE == BUint::<N>::ZERO {
                [2u8]
            } else {
                [3u8]
            };
            [&first_byte, &self.x.unwrap().num().to_be_bytes()[..]].concat()
        } else {
            [
                &[4u8],
                &self.x.unwrap().num().to_be_bytes()[..],
                &self.y.unwrap().num().to_be_bytes()[..],
            ]
            .concat()
        }
    }

    #[inline]
    pub fn parse(&self, sec_bin: Vec<u8>) -> Result<Self, String>
    where
        [(); BUint::<N>::BYTES_USIZE]:,
        FieldElement<M, N>: Sqrt,
    {
        if sec_bin[0] == 4u8 {
            let x = BUint::<N>::from_be_bytes(sec_bin[1..E::SEC_X_END].try_into().unwrap());
            let y =
                BUint::<N>::from_be_bytes(sec_bin[E::SEC_X_END..E::SEC_Y_END].try_into().unwrap());
            return Self::from_values(x, y);
        }
        let x =
            FieldElement::<M, N>::new(BUint::<N>::from_be_bytes(sec_bin[1..].try_into().unwrap()));
        let alpha = x.pow(BUint::<N>::THREE, false) + *E::B;
        let beta = alpha.sqrt();
        let (even_beta, odd_beta) = if beta.num() & BUint::<N>::ONE == BUint::<N>::ZERO {
            (beta, FieldElement::<M, N>::new(M::PRIME - beta.num()))
        } else {
            (FieldElement::<M, N>::new(M::PRIME - beta.num()), beta)
        };
        if sec_bin[0] == 2u8 {
            return Self::new(Some(x), Some(even_beta));
        } else {
            return Self::new(Some(x), Some(odd_beta));
        }
    }
}

impl<E, M, const N: usize> Add for Point<E, M, N>
where
    M: Modulus<N>,
    E: EllipticCurve<M, N>,
    [(); 2 * N]:,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        if self.is_infinity() {
            return rhs;
        }
        if rhs.is_infinity() {
            return self;
        }
        let x1 = self.x.unwrap();
        let y1 = self.y.unwrap();
        let x2 = rhs.x.unwrap();
        let y2 = rhs.y.unwrap();
        if x1 == x2 && y1 != y2 {
            return Self::new(None, None).unwrap();
        }
        let (x3, y3) = if x1 != x2 {
            let s = (y2 - y1) / (x2 - x1);
            let x3 = s.pow(BUint::<N>::TWO, false) - x1 - x2;
            let y3 = s * (x1 - x3) - y1;
            (x3, y3)
        } else if self == rhs {
            if y1 == E::FIELD_0 {
                return Self::new(None, None).unwrap();
            }
            let s = (*E::FIELD_3 * x1.pow(BUint::<N>::TWO, false) + *E::A) / (*E::FIELD_2 * y1);
            let x3 = s.pow(BUint::<N>::TWO, false) - *E::FIELD_2 * x1;
            let y3 = s * (x1 - x3) - y1;
            (x3, y3)
        } else {
            debug_assert!(
                false,
                "Could not add points {:?}, {:?}. This should be logically impossible.",
                self, rhs
            );
            return Self::INFINITY;
        };
        Self::new(Some(x3), Some(y3)).unwrap()
    }
}

impl<E, M, const N: usize> AddAssign for Point<E, M, N>
where
    M: Modulus<N>,
    E: EllipticCurve<M, N>,
    [(); 2 * N]:,
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<E, M, const N: usize> Mul<BUint<N>> for Point<E, M, N>
where
    M: Modulus<N>,
    E: EllipticCurve<M, N>,
    [(); 2 * N]:,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: BUint<N>) -> Self::Output {
        let mut coef = rhs;
        coef %= E::N;
        let mut current = self;
        let mut result = Self::INFINITY;
        while coef > BUint::<N>::ZERO {
            if coef & BUint::<N>::ONE == BUint::<N>::ONE {
                result += current;
            }
            current += current;
            coef >>= 1;
        }
        result
    }
}

macro_rules! point {
    ($name:ident, $curve_config: ident, $modulus: ident, $bits: expr, $a_str:expr, $b_str:expr, $gx_str:expr, $gy_str:expr, $n_str:expr, $radix:expr) => {
        #[derive(PartialEq, Debug, Clone, Copy)]
        pub struct $curve_config;

        impl EllipticCurve<$modulus, { $bits / 64 }> for $curve_config {
            const A: Lazy<FieldElement<$modulus, { $bits / 64 }>> = Lazy::new(|| {
                FieldElement::<$modulus, { $bits / 64 }>::new(
                    BUint::<{ $bits / 64 }>::parse_str_radix($a_str, $radix),
                )
            });
            const B: Lazy<FieldElement<$modulus, { $bits / 64 }>> = Lazy::new(|| {
                FieldElement::<$modulus, { $bits / 64 }>::new(
                    BUint::<{ $bits / 64 }>::parse_str_radix($b_str, $radix),
                )
            });
            const GX: Lazy<FieldElement<$modulus, { $bits / 64 }>> = Lazy::new(|| {
                FieldElement::<$modulus, { $bits / 64 }>::new(
                    BUint::<{ $bits / 64 }>::parse_str_radix($gx_str, $radix),
                )
            });
            const GY: Lazy<FieldElement<$modulus, { $bits / 64 }>> = Lazy::new(|| {
                FieldElement::<$modulus, { $bits / 64 }>::new(
                    BUint::<{ $bits / 64 }>::parse_str_radix($gy_str, $radix),
                )
            });
            const N: BUint<{ $bits / 64 }> =
                BUint::<{ $bits / 64 }>::parse_str_radix($n_str, $radix);
            const FIELD_0: FieldElement<$modulus, { $bits / 64 }> =
                FieldElement::<$modulus, { $bits / 64 }>::FIELD_0;
            const FIELD_2: Lazy<FieldElement<$modulus, { $bits / 64 }>> = Lazy::new(|| {
                FieldElement::<$modulus, { $bits / 64 }>::new(BUint::<{ $bits / 64 }>::TWO)
            });
            const FIELD_3: Lazy<FieldElement<$modulus, { $bits / 64 }>> = Lazy::new(|| {
                FieldElement::<$modulus, { $bits / 64 }>::new(BUint::<{ $bits / 64 }>::THREE)
            });

            const SEC_X_END: usize = BUint::<{ $bits / 64 }>::BYTES as usize + 1;
            const SEC_Y_END: usize = 2 * BUint::<{ $bits / 64 }>::BYTES as usize + 1;
        }

        pub type $name = crate::ecc::elliptic_curve::Point<$curve_config, $modulus, { $bits / 64 }>;
    };
}
