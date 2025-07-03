use crate::curves::secp256k1::Secp256k1Params;
use hmac::{Hmac, Mac};
use k256::elliptic_curve::bigint::Encoding;
use k256::elliptic_curve::PrimeField;
use k256::{Scalar, U256};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub fn generate_deterministic_nonce(private_key: &[u8], message_hash: &[u8]) -> Scalar {
    let order = Secp256k1Params::order();

    // RFC 6979 algorithm
    let mut v = [0x01u8; 32];
    let mut k = [0x00u8; 32];

    // K = HMAC_K(V || 0x00 || private_key || message_hash)
    let mut mac = HmacSha256::new_from_slice(&k).unwrap();
    mac.update(&v);
    mac.update(&[0x00]);
    mac.update(private_key);
    mac.update(message_hash);
    k = mac.finalize().into_bytes().into();

    // V = HMAC_K(V)
    let mut mac = HmacSha256::new_from_slice(&k).unwrap();
    mac.update(&v);
    v = mac.finalize().into_bytes().into();

    // K = HMAC_K(V || 0x01 || private_key || message_hash)
    let mut mac = HmacSha256::new_from_slice(&k).unwrap();
    mac.update(&v);
    mac.update(&[0x01]);
    mac.update(private_key);
    mac.update(message_hash);
    k = mac.finalize().into_bytes().into();

    // V = HMAC_K(V)
    let mut mac = HmacSha256::new_from_slice(&k).unwrap();
    mac.update(&v);
    v = mac.finalize().into_bytes().into();

    // Generate candidate nonce
    loop {
        let mut mac = HmacSha256::new_from_slice(&k).unwrap();
        mac.update(&v);
        v = mac.finalize().into_bytes().into();

        let candidate = U256::from_be_bytes(v);
        if candidate > U256::ZERO && candidate < order {
            return Scalar::from_repr(v.into()).unwrap();
        }

        // K = HMAC_K(V || 0x00)
        let mut mac = HmacSha256::new_from_slice(&k).unwrap();
        mac.update(&v);
        mac.update(&[0x00]);
        k = mac.finalize().into_bytes().into();

        // V = HMAC_K(V)
        let mut mac = HmacSha256::new_from_slice(&k).unwrap();
        mac.update(&v);
        v = mac.finalize().into_bytes().into();
    }
}
