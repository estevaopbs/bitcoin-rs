use bnum::cast::As;
pub use bnum::BUint;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Sub};
use std::usize;

pub trait Modulus<const N: usize>: PartialEq + Debug + Clone + Copy
where
    [(); 2 * N]:,
{
    const PRIME: BUint<N>;
    const BIG_PRIME: BUint<{ 2 * N }>;

    fn to_big(x: BUint<N>) -> BUint<{ 2 * N }> {
        x.as_()
    }

    fn from_big(x: BUint<{ 2 * N }>) -> BUint<N> {
        x.as_()
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct FieldElement<M: Modulus<N>, const N: usize>
where
    [(); 2 * N]:,
{
    num: BUint<N>,
    _marker: PhantomData<M>,
}

impl<M: Modulus<N>, const N: usize> FieldElement<M, N>
where
    [(); 2 * N]:,
{
    pub const FIELD_0: FieldElement<M, N> = FieldElement {
        num: BUint::<N>::ZERO,
        _marker: PhantomData,
    };

    #[inline]
    pub fn new(num: BUint<N>) -> Self {
        Self {
            num: {
                if num < M::PRIME {
                    num
                } else {
                    num % M::PRIME
                }
            },
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn num(&self) -> BUint<N> {
        self.num
    }

    #[inline]
    pub fn pow(&self, exp: BUint<N>, is_negative: bool) -> Self {
        Self::new(Self::mod_pow(self.num, exp, is_negative, M::PRIME))
    }

    #[inline]
    pub fn mod_pow(
        base: BUint<N>,
        mut exp: BUint<N>,
        is_negative: bool,
        modulus: BUint<N>,
    ) -> BUint<N>
    where
        [(); 2 * N]:,
    {
        if is_negative {
            let mut big_exp = M::to_big(exp);
            big_exp = M::BIG_PRIME
                - BUint::<{ 2 * N }>::ONE
                - big_exp % (M::BIG_PRIME - BUint::<{ 2 * N }>::ONE);
            exp = M::from_big(big_exp);
        }
        let mut result = BUint::<{ 2 * N }>::ONE;
        let mut base = M::to_big(base);

        let modulus = M::to_big(modulus);
        while exp > BUint::<N>::ZERO {
            if exp % BUint::<N>::TWO == BUint::<N>::ONE {
                result = (result * base) % modulus;
            }
            base = (base * base) % modulus;
            exp >>= 1;
        }
        M::from_big(result)
    }
}

impl<M: Modulus<N>, const N: usize> Add for FieldElement<M, N>
where
    [(); 2 * N]:,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        let num = (M::to_big(self.num) + M::to_big(rhs.num)) % M::BIG_PRIME;
        Self::new(M::from_big(num))
    }
}

impl<M: Modulus<N>, const N: usize> Sub for FieldElement<M, N>
where
    [(); 2 * N]:,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        let num = if self.num < rhs.num {
            self.num + (M::PRIME - rhs.num)
        } else {
            self.num - rhs.num
        };
        Self::new(num)
    }
}

impl<M: Modulus<N>, const N: usize> Mul for FieldElement<M, N>
where
    [(); 2 * N]:,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        let num = (M::to_big(self.num) * M::to_big(rhs.num)) % M::BIG_PRIME;
        Self::new(M::from_big(num))
    }
}

impl<M: Modulus<N>, const N: usize> Div for FieldElement<M, N>
where
    [(); 2 * N]:,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self {
        let inv = rhs.pow(M::PRIME - BUint::<N>::TWO, false);
        self * inv
    }
}

pub trait Sqrt {
    fn sqrt(&self) -> Self;
}

pub fn p3mod4<M: Modulus<N>, const N: usize>(x: FieldElement<M, N>) -> FieldElement<M, N>
where
    [(); 2 * N]:,
{
    x.pow((M::PRIME + BUint::<N>::ONE) / BUint::<N>::FOUR, false)
}

pub fn p5mod8<M: Modulus<N>, const N: usize>(x: FieldElement<M, N>) -> FieldElement<M, N>
where
    [(); 2 * N]:,
{
    x.pow((M::PRIME + BUint::<N>::THREE) / BUint::<N>::EIGHT, false)
}

pub fn tonelli_shanks<M: Modulus<N>, const N: usize>(x: FieldElement<M, N>) -> FieldElement<M, N>
where
    [(); 2 * N]:,
{
    let zero = BUint::<N>::ZERO;
    let one = BUint::<N>::ONE;
    let two = BUint::<N>::TWO;

    if x.num == zero {
        return x;
    }

    let mut q = M::PRIME - one;
    let mut s = zero;
    while q % two == zero {
        q = q / two;
        s = s + one;
    }

    let mut z = one + one;
    while FieldElement::<M, N>::new(z)
        .pow((M::PRIME - one) / two, false)
        .num
        != M::PRIME - one
    {
        z = z + one;
    }

    let mut m = s;
    let mut c = FieldElement::<M, N>::new(z).pow(q, false);
    let mut t = x.pow(q, false);
    let mut r = x.pow((q + one) / two, false);

    while t.num != one {
        let mut i = zero;
        let mut temp = t;
        while temp.num != one {
            temp = temp.pow(two, false);
            i = i + one;
        }

        let b = c.pow(two.pow((m - i - one).as_::<usize>() as u32), false);
        r = r * b;
        c = b.pow(two, false);
        t = t * c;
        m = i;
    }

    r
}

macro_rules! field_element {
    ($name:ident, $config_name:ident, $bits:expr, $prime_str:expr, $radix:expr, $sqrt_method:ident) => {
        #[derive(PartialEq, Debug, Clone, Copy)]
        pub struct $config_name;

        impl Modulus<{ $bits / 64 }> for $config_name {
            const PRIME: bnum::BUint<{ $bits / 64 }> =
                bnum::BUint::<{ $bits / 64 }>::parse_str_radix($prime_str, $radix);
            const BIG_PRIME: bnum::BUint<{ $bits / 32 }> =
                bnum::BUint::<{ $bits / 32 }>::parse_str_radix($prime_str, $radix);
        }

        pub type $name = crate::ecc::finite_field::FieldElement<$config_name, { $bits / 64 }>;

        impl Sqrt for $name {
            fn sqrt(&self) -> Self {
                crate::ecc::finite_field::$sqrt_method(*self)
            }
        }
    };
}
