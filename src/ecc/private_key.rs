macro_rules! private_key {
    ($name:ident, $field_type:ty, $sig_type:ty, $point_type:ty, $num_type:ty, $bnum_type:ty, $vec_size:expr, $hasher:ty) => {
        #[derive(Debug)]
        pub struct $name {
            secret: $field_type,
            point: $point_type,
        }

        impl $name {
            pub fn new(secret: $field_type) -> Self {
                Self {
                    secret,
                    point: *<$point_type>::G * secret.num(),
                }
            }

            pub fn from_value(secret: $num_type) -> Self {
                Self::new(<$field_type>::new(secret))
            }

            pub fn secret(&self) -> $field_type {
                self.secret
            }

            pub fn point(&self) -> $point_type {
                self.point
            }

            fn to_big(num: $num_type) -> $bnum_type {
                <$bnum_type as BTryFrom<$num_type>>::try_from(num).unwrap()
            }

            fn from_big(num: $bnum_type) -> $num_type {
                <$num_type as BTryFrom<$bnum_type>>::try_from(num).unwrap()
            }

            pub fn sign(&self, z: $num_type) -> $sig_type {
                let k = self.deterministic_k(z);
                let r = (*<$point_type>::G * k).x().unwrap().num();
                let k_inv = <$field_type>::mod_pow(
                    k,
                    *<$point_type>::N - <$num_type>::TWO,
                    false,
                    *<$point_type>::N,
                );
                let mut s = Self::from_big(
                    (Self::to_big(z) + Self::to_big(r) * Self::to_big(self.secret.num()))
                        * Self::to_big(k_inv)
                        % Self::to_big(*<$point_type>::N),
                );
                if s > *<$point_type>::N / <$num_type>::TWO {
                    s = *<$point_type>::N - s;
                }
                <$sig_type>::from_values(r, s)
            }

            fn deterministic_k(&self, mut z: $num_type) -> $num_type {
                let k = [0u8; $vec_size];
                let v = [1u8; $vec_size];
                if z > *<$point_type>::N {
                    z -= *<$point_type>::N;
                }
                let z_bytes = z.to_be_bytes();
                let secret_bytes = self.secret.num().to_be_bytes();
                let mut k = Hmac::<$hasher>::new_from_slice(&k)
                    .unwrap()
                    .chain_update(&[&v[..], &[0u8], &secret_bytes[..], &z_bytes[..]].concat())
                    .finalize()
                    .into_bytes();
                let mut v = Hmac::<$hasher>::new_from_slice(&k)
                    .unwrap()
                    .chain_update(&v[..])
                    .finalize()
                    .into_bytes();
                k = Hmac::<$hasher>::new_from_slice(&k)
                    .unwrap()
                    .chain_update(&[&v[..], &[1u8]].concat())
                    .finalize()
                    .into_bytes();
                v = Hmac::<$hasher>::new_from_slice(&k)
                    .unwrap()
                    .chain_update(&v[..])
                    .finalize()
                    .into_bytes();
                loop {
                    v = Hmac::<$hasher>::new_from_slice(&k)
                        .unwrap()
                        .chain_update(&v[..])
                        .finalize()
                        .into_bytes();
                    let candidate = <$num_type>::from_be_bytes(v.try_into().unwrap());
                    if candidate >= <$num_type>::ONE && candidate < *<$point_type>::N {
                        return candidate;
                    }
                    k = Hmac::<$hasher>::new_from_slice(&k)
                        .unwrap()
                        .chain_update(&[&v[..], &[0u8]].concat())
                        .finalize()
                        .into_bytes();
                    v = Hmac::<$hasher>::new_from_slice(&k)
                        .unwrap()
                        .chain_update(&v[..])
                        .finalize()
                        .into_bytes();
                }
            }
        }
    };
}
