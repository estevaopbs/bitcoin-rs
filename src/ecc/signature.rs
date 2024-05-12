macro_rules! signature {
    ($name:ident, $field_type:ty, $num_type:ty) => {
        #[derive(Debug)]
        pub struct $name {
            r: $field_type,
            s: $field_type,
        }

        impl $name {
            pub fn new(r: $field_type, s: $field_type) -> Self {
                Self { r, s }
            }

            pub fn from_values(r: $num_type, s: $num_type) -> Self {
                Self::new(<$field_type>::new(r), <$field_type>::new(s))
            }

            pub fn r(&self) -> $field_type {
                self.r
            }

            pub fn s(&self) -> $field_type {
                self.s
            }

            pub fn der(&self) -> Vec<u8> {
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
                    if bin[0] >= 128u8 {
                        bin.splice(0..0, [0u8].iter().cloned());
                    }
                    result.push(2u8);
                    result.push(bin.len() as u8);
                    result.extend(bin);
                }
                result
            }
        }
    };
}
