use crate::core::{S256Field, S256Point, S256PrivateKey, S256Signature};
use bnum::types::{U256, U512};
use bnum::BTryFrom;
use rand::rngs::OsRng;
use rand::RngCore;

#[test]
fn test_order() {
    assert_eq!(*S256Point::G * *S256Point::N, S256Point::INFINITY)
}

#[test]
fn test_pubpoint() {
    let points = [
        (
            U256::from_digit(7),
            U256::parse_str_radix(
                "5CBDF0646E5DB4EAA398F365F2EA7A0E3D419B7E0330E39CE92BDDEDCAC4F9BC",
                16,
            ),
            U256::parse_str_radix(
                "6AEBCA40BA255960A3178D6D861A54DBA813D0B813FDE7B5A5082628087264DA",
                16,
            ),
        ),
        (
            U256::from_digit(1485),
            U256::parse_str_radix(
                "C982196A7466FBBBB0E27A940B6AF926C1A74D5AD07128C82824A11B5398AFDA",
                16,
            ),
            U256::parse_str_radix(
                "7A91F9EAE64438AFB9CE6448A1C133DB2D8FB9254E4546B6F001637D50901F55",
                16,
            ),
        ),
        (
            U256::from_digit(2).pow(128),
            U256::parse_str_radix(
                "8F68B9D2F63B5F339239C1AD981F162EE88C5678723EA3351B7B444C9EC4C0DA",
                16,
            ),
            U256::parse_str_radix(
                "662A9F2DBA063986DE1D90C2B6BE215DBBEA2CFE95510BFDF23CBF79501FFF82",
                16,
            ),
        ),
        (
            U256::from_digit(2).pow(240) + U256::from_digit(2).pow(31),
            U256::parse_str_radix(
                "9577FF57C8234558F293DF502CA4F09CBC65A6572C842B39B366F21717945116",
                16,
            ),
            U256::parse_str_radix(
                "10B49C67FA9365AD7B90DAB070BE339A1DAF9052373EC30FFAE4F72D5E66D053",
                16,
            ),
        ),
    ];
    for (secret, x, y) in points {
        let point = S256Point::from_values(x, y).unwrap();
        assert_eq!(*S256Point::G * secret, point);
    }
}

#[test]
fn test_verify() {
    let point = S256Point::from_values(
        U256::parse_str_radix(
            "887387E452B8EACC4ACFDE10D9AAF7F6D9A0F975AABB10D006E4DA568744D06C",
            16,
        ),
        U256::parse_str_radix(
            "61DE6D95231CD89026E286DF3B6AE4A894A3378E393E93A0F45B666329A0AE34",
            16,
        ),
    )
    .unwrap();
    let z = U256::parse_str_radix(
        "EC208BAA0FC1C19F708A9CA96FDEFF3AC3F230BB4A7BA4AEDE4942AD003C0F60",
        16,
    );
    let r = U256::parse_str_radix(
        "AC8D1C87E51D0D441BE8B3DD5B05C8795B48875DFFE00B7FFCFAC23010D3A395",
        16,
    );
    let s = U256::parse_str_radix(
        "68342CEFF8935EDEDD102DD876FFD6BA72D6A427A3EDB13D26EB0781CB423C4",
        16,
    );
    let sig = S256Signature::from_values(r, s);
    assert!(point.verify(z, sig));
    let z = U256::parse_str_radix(
        "7C076FF316692A3D7EB3C3BB0F8B1488CF72E1AFCD929E29307032997A838A3D",
        16,
    );
    let r = U256::parse_str_radix(
        "EFF69EF2B1BD93A66ED5219ADD4FB51E11A840F404876325A1E8FFE0529A2C",
        16,
    );
    let s = U256::parse_str_radix(
        "C7207FEE197D27C618AEA621406F6BF5EF6FCA38681D82B2F06FDDBDCE6FEAB6",
        16,
    );
    let sig = S256Signature::from_values(r, s);
    assert!(point.verify(z, sig));
}

#[test]
fn test_sign() {
    let mut rand_array = [0u8; 32];
    OsRng.fill_bytes(rand_array.as_mut());
    let secret = U256::from_radix_be(&rand_array, 256).unwrap();
    let pk = S256PrivateKey::from_value(secret);
    let mut rand_array = [0u8; 32];
    OsRng.fill_bytes(rand_array.as_mut());
    let z = U256::from_radix_be(&rand_array, 256).unwrap();
    let sig = pk.sign(z);
    assert!(pk.point().verify(z, sig));
}

// Failed test
#[test]
fn test_wif() {
    let pk =
        S256PrivateKey::from_value(S256Field::from_big(U512::TWO.pow(256) - U512::TWO.pow(199)));
    let expected = "L5oLkpV3aqBJ4BgssVAsax1iRa77G5CVYnv9adQ6Z87te7TyUdSC";
    assert_eq!(pk.wif(true, false), expected);
}
