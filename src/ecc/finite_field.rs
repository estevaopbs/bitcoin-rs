macro_rules! field_element {
    ($name:ident, $num_type:ty, $bnum_type:ty, $prime_val:expr) => {
        pub trait Sqrt {
            fn sqrt(&self) -> Self;
        }

        #[derive(PartialEq, Debug, Clone, Copy)]
        pub struct $name {
            num: $num_type,
        }

        impl $name {
            pub const PRIME: Lazy<$num_type> = Lazy::new(|| $prime_val);

            pub const BIG_PRIME: Lazy<$bnum_type> =
                Lazy::new(|| <$bnum_type as BTryFrom<$num_type>>::try_from(*Self::PRIME).unwrap());

            pub fn new(num: $num_type) -> Self {
                Self {
                    num: num % *Self::PRIME,
                }
            }

            pub fn pow(&self, exp: $num_type, is_negative: bool) -> Self {
                Self::new(Self::mod_pow(self.num, exp, is_negative, *Self::PRIME))
            }

            pub fn num(&self) -> $num_type {
                self.num
            }

            fn big_num(&self) -> $bnum_type {
                Self::to_big(self.num)
            }

            pub(crate) fn from_big(num: $bnum_type) -> $num_type {
                <$num_type as BTryFrom<$bnum_type>>::try_from(num).unwrap()
            }

            pub(crate) fn to_big(num: $num_type) -> $bnum_type {
                <$bnum_type as BTryFrom<$num_type>>::try_from(num).unwrap()
            }

            pub fn mod_pow(
                base: $num_type,
                mut exp: $num_type,
                is_negative: bool,
                modulus: $num_type,
            ) -> $num_type {
                if is_negative {
                    let mut big_exp = Self::to_big(exp);
                    big_exp = *Self::BIG_PRIME
                        - <$bnum_type>::ONE
                        - big_exp % (*Self::BIG_PRIME - <$bnum_type>::ONE);
                    exp = Self::from_big(big_exp);
                }
                let mut result = <$bnum_type>::ONE;
                let mut base = Self::to_big(base);

                let modulus = Self::to_big(modulus);
                while exp > <$num_type>::ZERO {
                    if exp % <$num_type>::TWO == <$num_type>::ONE {
                        result = (result * base) % modulus;
                    }
                    base = (base * base) % modulus;
                    exp >>= 1;
                }
                Self::from_big(result)
            }
        }

        impl Add for $name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                let num = (self.big_num() + Self::to_big(rhs.num)) % *Self::BIG_PRIME;
                Self::new(Self::from_big(num))
            }
        }

        impl Sub for $name {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                let num = if self.num < rhs.num {
                    self.num + (*Self::PRIME - rhs.num)
                } else {
                    self.num - rhs.num
                };
                Self::new(num)
            }
        }

        impl Mul for $name {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                let num = (self.big_num() * Self::to_big(rhs.num)) % *Self::BIG_PRIME;
                Self::new(Self::from_big(num))
            }
        }

        impl Div for $name {
            type Output = Self;

            fn div(self, rhs: Self) -> Self::Output {
                let other_inverse = rhs.pow(*Self::PRIME - <$num_type>::TWO, false);
                self * other_inverse
            }
        }
    };
}
