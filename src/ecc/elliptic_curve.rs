macro_rules! point {
    ($name:ident, $field_type:ty, $signatre_type:ty, $num_type:ty, $a:expr, $b:expr, $gx:expr, $gy:expr, $n:expr) => {
        #[derive(PartialEq, Debug, Clone, Copy)]
        pub struct $name {
            x: Option<$field_type>,
            y: Option<$field_type>,
        }

        impl $name {
            pub const A: Lazy<$field_type> = Lazy::new(|| <$field_type>::new($a));

            pub const B: Lazy<$field_type> = Lazy::new(|| <$field_type>::new($b));

            pub const G: Lazy<Self> = Lazy::new(|| Self::from_values($gx, $gy).unwrap());

            pub const N: Lazy<$num_type> = Lazy::new(|| $n);

            pub const FIELD_0: Lazy<$field_type> =
                Lazy::new(|| <$field_type>::new(<$num_type>::ZERO));

            pub const FIELD_2: Lazy<$field_type> =
                Lazy::new(|| <$field_type>::new(<$num_type>::TWO));

            pub const FIELD_3: Lazy<$field_type> =
                Lazy::new(|| <$field_type>::new(<$num_type>::THREE));

            pub const INFINITY: Self = Self { x: None, y: None };

            const SEC_X_END: usize = <$num_type>::BYTES as usize + 1;

            const SEC_Y_END: usize = 2 * <$num_type>::BYTES as usize + 1;

            pub fn new(x: Option<$field_type>, y: Option<$field_type>) -> Result<Self, String> {
                if x.is_none() && y.is_none() {
                    return Ok(Self::INFINITY);
                }
                if x.is_none() || y.is_none() {
                    return Err("Either both x and y must be None or neither".to_string());
                }
                let x = x.unwrap();
                let y = y.unwrap();
                if y.pow(<$num_type>::TWO, false)
                    != x.pow(<$num_type>::THREE, false) + *Self::A * x + *Self::B
                {
                    return Err(format!("({:?}, {:?}) is not on the curve", x, y));
                }
                Ok(Self {
                    x: Some(x),
                    y: Some(y),
                })
            }

            pub fn from_values(x: $num_type, y: $num_type) -> Result<Self, String> {
                Self::new(Some(<$field_type>::new(x)), Some(<$field_type>::new(y)))
            }

            pub fn x(&self) -> Option<$field_type> {
                self.x
            }

            pub fn y(&self) -> Option<$field_type> {
                self.y
            }

            pub fn is_infinity(&self) -> bool {
                self.x.is_none()
            }

            pub fn verify(&self, z: $num_type, sig: $signatre_type) -> bool {
                let s_inv = <$field_type>::mod_pow(
                    sig.s().num(),
                    *Self::N - <$num_type>::TWO,
                    false,
                    *Self::N,
                );
                let u = <$field_type>::from_big(
                    <$field_type>::to_big(z) * <$field_type>::to_big(s_inv)
                        % <$field_type>::to_big(*Self::N),
                );
                let v = <$field_type>::from_big(
                    <$field_type>::to_big(sig.r().num()) * <$field_type>::to_big(s_inv)
                        % <$field_type>::to_big(*Self::N),
                );
                let total = *Self::G * u + *self * v;
                total.x.unwrap() == sig.r()
            }

            pub fn sec(&self, compressed: bool) -> Vec<u8> {
                if compressed {
                    let first_byte =
                        if self.y.unwrap().num() & <$num_type>::ONE == <$num_type>::ZERO {
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

            pub fn parse(&self, sec_bin: Vec<u8>) -> Result<Self, String> {
                if sec_bin[0] == 4u8 {
                    let x =
                        <$num_type>::from_be_bytes(sec_bin[1..Self::SEC_X_END].try_into().unwrap());
                    let y = <$num_type>::from_be_bytes(
                        sec_bin[Self::SEC_X_END..Self::SEC_Y_END]
                            .try_into()
                            .unwrap(),
                    );
                    return Self::from_values(x, y);
                }
                let x = <$field_type>::new(<$num_type>::from_be_bytes(
                    sec_bin[1..].try_into().unwrap(),
                ));
                let alpha = x.pow(<$num_type>::THREE, false) + *Self::B;
                let beta = alpha.sqrt();
                let (even_beta, odd_beta) = if beta.num() & <$num_type>::ONE == <$num_type>::ZERO {
                    (beta, <$field_type>::new(*<$field_type>::PRIME - beta.num()))
                } else {
                    (<$field_type>::new(*<$field_type>::PRIME - beta.num()), beta)
                };
                if sec_bin[0] == 2u8 {
                    return Self::new(Some(x), Some(even_beta));
                } else {
                    return Self::new(Some(x), Some(odd_beta));
                }
            }
        }

        impl Add for $name {
            type Output = Self;

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
                    let x3 = s.pow(<$num_type>::TWO, false) - x1 - x2;
                    let y3 = s * (x1 - x3) - y1;
                    (x3, y3)
                } else if self == rhs {
                    if y1 == *Self::FIELD_0 {
                        return Self::new(None, None).unwrap();
                    }
                    let s = (*Self::FIELD_3 * x1.pow(<$num_type>::TWO, false) + *Self::A)
                        / (*Self::FIELD_2 * y1);
                    let x3 = s.pow(<$num_type>::TWO, false) - *Self::FIELD_2 * x1;
                    let y3 = s * (x1 - x3) - y1;
                    (x3, y3)
                } else {
                    panic!("Could not add points {:?}, {:?}", self, rhs);
                };
                Self::new(Some(x3), Some(y3)).unwrap()
            }
        }

        impl AddAssign<$name> for $name {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        impl Mul<$num_type> for $name {
            type Output = Self;

            fn mul(self, rhs: $num_type) -> Self::Output {
                let mut coef = rhs;
                coef %= *Self::N;
                let mut current = self;
                let mut result = Self::INFINITY;
                while coef > <$num_type>::ZERO {
                    if coef & <$num_type>::ONE == <$num_type>::ONE {
                        result += current;
                    }
                    current += current;
                    coef >>= 1;
                }
                result
            }
        }
    };
}
