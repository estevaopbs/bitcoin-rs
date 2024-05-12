pub const BASE58_ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

macro_rules! encode_base58 {
    ($name:ident, $num_type:ty) => {
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
