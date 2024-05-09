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

            #[allow(dead_code)]
            pub fn from_values(r: $num_type, s: $num_type) -> Self {
                Self::new(<$field_type>::new(r), <$field_type>::new(s))
            }

            pub fn r(&self) -> $field_type {
                self.r
            }

            pub fn s(&self) -> $field_type {
                self.s
            }
        }
    };
}
