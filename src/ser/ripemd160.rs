macro_rules! hash160 {
    ($name:ident, $hasher:ty) => {
        pub fn $name(s: Vec<u8>) -> Vec<u8> {
            let mut hasher = <$hasher>::new();
            hasher.update(&s);
            let hash = hasher.finalize().to_vec();
            let mut hasher160 = Ripemd160::new();
            hasher160.update(&hash);
            hasher160.finalize().to_vec()
        }
    };
}
