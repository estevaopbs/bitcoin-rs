macro_rules! encode_base58 {
    ($name:ident, $num_type:ty, $bytes:expr) => {
        pub const BASE58_ALPHABET: &str =
            "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

        pub fn $name(s: Vec<u8>) -> String {
            let num_58 = <$num_type>::from_digit(58);
            let zeros = s.iter().take_while(|&&c| c == 0).count();
            let prefix = "1".repeat(zeros);
            let mut extended_s = vec![0; $bytes - s.len()];
            extended_s.extend_from_slice(&s);
            let mut num = <$num_type>::from_be_bytes(extended_s.try_into().unwrap());
            let mut result = String::new();
            while num > <$num_type>::ZERO {
                let rem: usize = (num % num_58).try_into().unwrap();
                result.push(BASE58_ALPHABET.chars().nth(rem).unwrap());
                num /= num_58;
            }
            result.push_str(&prefix);
            result.chars().rev().collect()
        }
    };
}

macro_rules! encode_base58_checksum {
    ($name:ident, $hasher_fn:path, $encode_base58:path) => {
        pub fn $name(mut s: Vec<u8>) -> String {
            let checksum = $hasher_fn(&s);
            s.extend_from_slice(&checksum[..4]);
            $encode_base58(s)
        }
    };
}
