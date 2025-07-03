// Step 3: Pick message m and hash it to produce h
use k256::elliptic_curve::bigint::{Encoding, U256};
use sha2::{Digest, Sha256};

pub fn hash_message(message: &[u8]) -> U256 {
    let hash = Sha256::digest(message);
    U256::from_be_bytes(hash.into())
}
