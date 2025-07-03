// Step 3: Pick message m and hash it to produce h
use crate::curves::secp256k1::Secp256k1Params;
use k256::elliptic_curve::bigint::{Encoding, NonZero, U256};
use sha2::{Digest, Sha256};

pub fn hash_message(message: &[u8]) -> U256 {
    let hash = Sha256::digest(message);
    let h = U256::from_be_bytes(hash.into());
    let order = Secp256k1Params::order();
    let order_nz = NonZero::new(order).unwrap();
    h % order_nz
}
