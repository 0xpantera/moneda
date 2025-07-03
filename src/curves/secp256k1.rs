use k256::elliptic_curve::bigint::U256;
use k256::elliptic_curve::Curve;
use k256::{ProjectivePoint, Secp256k1};
use num_bigint::BigInt;

pub struct Secp256k1Params;

impl Secp256k1Params {
    // Get curve order from k256
    pub fn order() -> U256 {
        Secp256k1::ORDER
    }

    // Field prime (for reference, not directly used in ECDSA)
    pub fn field_prime() -> BigInt {
        BigInt::parse_bytes(
            b"115792089237316195423570985008687907853269984665640564039457584007908834671663",
            10,
        )
        .unwrap()
    }

    // Generator point G
    pub fn generator() -> ProjectivePoint {
        k256::ProjectivePoint::GENERATOR
    }
}
