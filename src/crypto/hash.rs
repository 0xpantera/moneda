// Step 3: Pick message m and hash it to produce h (reduced mod n)
use crate::curves::secp256k1::Secp256k1Params;
use k256::elliptic_curve::bigint::{Encoding, NonZero, U256};
use sha2::{Digest, Sha256};

pub fn hash_message(message: &[u8]) -> U256 {
    let hash = Sha256::digest(message);
    let h = U256::from_be_bytes(hash.into());

    // Reduce mod n as required by SEC 1 spec
    // "Convert the bit string to an integer e; then reduce e modulo n"
    let order = Secp256k1Params::order();
    let order_nonzero = NonZero::new(order).unwrap(); // Safe: curve order is never zero

    h % order_nonzero
}
