use ripemd::Ripemd160;
use sha2::Sha256;

chained_hash!(DoubleSha256, Sha256, Sha256, 1);

base58!(Sha256Base58, DoubleSha256, 1, Sha256, Sha256, 512);

chained_hash!(Sha256Ripemd160, Sha256, Ripemd160, 1);
