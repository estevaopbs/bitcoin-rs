macro_rules! hash {
    ($name:ident, $hasher:ty, $rounds:expr) => {
        fn $name(data: &[u8]) -> Vec<u8> {
            let mut hasher = <$hasher>::new();
            hasher.update(data);
            let mut result = hasher.finalize().to_vec();
            for _ in 1..$rounds {
                let mut hasher = <$hasher>::new();
                hasher.update(&result);
                result = hasher.finalize().to_vec();
            }
            result
        }
    };
}
