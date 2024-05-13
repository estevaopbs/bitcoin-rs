macro_rules! encode_base58 {
    ($name:ident, $num_type:ty) => {
        pub const BASE58_ALPHABET: &str =
            "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

        pub fn $name(s: vec<u8>) -> String {
            let num_58 = <$num_type>::from(58);
            let zeros = s.iter().take_while(|&&c| c == 0).count();
            let prefix = "1".repeat(zeros);
            let mut num = $num_type::from_be_bytes(s);
            let mut result = String::new();
            while num > 0 {
                let rem = num % num_58;
                result.push(BASE58_ALPHABET.chars().nth(rem as usize).unwrap());
                num /= num_58;
            }
            result.push_str(&prefix);
            result.chars().rev().collect()
        }
    };
}

macro_rules! encode_base58_checksum {
    ($name:ident, $num_type:ty, $hasher:ty, $encode_base58:path) => {
        pub fn $name(s: vec<u8>) -> String {
            let checksum = Hmac::<$hasher>::new_from_slice(&s)
                .unwrap()
                .finalize()
                .into_bytes();
            let mut s = s.clone();
            s.extend_from_slice(&checksum[..4]);
            $encode_base58(s)
        }
    };
}
