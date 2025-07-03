use crate::crypto::keys::PrivateKey;
use crate::crypto::rfc6979::generate_deterministic_nonce;
use crate::curves::secp256k1::Secp256k1Params;
use crate::errors::EcdsaError;
use k256::elliptic_curve::bigint::{Encoding, NonZero, U256};
use k256::elliptic_curve::point::AffineCoordinates;
use k256::elliptic_curve::PrimeField;
use k256::{ProjectivePoint, Scalar};

#[derive(Debug, Clone)]
pub struct Signature {
    pub r: U256,
    pub s: U256,
}

impl PrivateKey {
    // Step 4: Sign message using private key and RFC 6979 deterministic nonce
    pub fn sign(&self, message_hash: &U256) -> Result<Signature, EcdsaError> {
        let order = Secp256k1Params::order();
        let order_nonzero = NonZero::new(order).unwrap();

        // Generate RFC 6979 deterministic nonce (no retry loop needed)
        let privkey_bytes = self.as_u256().to_be_bytes();
        let hash_bytes = message_hash.to_be_bytes();
        let k = generate_deterministic_nonce(&privkey_bytes, &hash_bytes);

        // Calculate R = k * G (using k256 for point multiplication)
        let r_point = Secp256k1Params::generator() * k;

        // Get x-coordinate: r = R.x mod n (RFC compliant)
        let r_affine = r_point.to_affine();
        let x_coord = r_affine.x();
        let mut x_bytes = [0u8; 32];
        x_bytes.copy_from_slice(&x_coord);
        let r_raw = U256::from_be_bytes(x_bytes);
        let r = r_raw % order_nonzero; // Proper r = (k·G).x mod n

        // Calculate s = k^(-1) * (h + r * privkey) mod n
        // Convert message hash and r to scalars for arithmetic
        let h_bytes: [u8; 32] = message_hash.to_be_bytes();
        let h_scalar = Scalar::from_repr(h_bytes.into()).unwrap();

        let r_bytes: [u8; 32] = r.to_be_bytes();
        let r_scalar = Scalar::from_repr(r_bytes.into()).unwrap();

        // Get private key as scalar
        let privkey_bytes = self.as_u256().to_be_bytes();
        let privkey_scalar = Scalar::from_repr(privkey_bytes.into()).unwrap();

        // Calculate k^(-1) mod n
        let k_inv = k.invert().unwrap();

        // Calculate s = k^(-1) * (h + r * privkey) mod n
        let s_scalar = k_inv * (h_scalar + r_scalar * privkey_scalar);
        let s_bytes = s_scalar.to_bytes();
        let s_raw = U256::from_be_bytes(s_bytes.into());

        // Low-s normalization (BIP-62) - prevent signature malleability
        let half_order = order >> 1;
        let s = if s_raw > half_order {
            order.wrapping_sub(&s_raw) // If s > n/2, use n - s instead
        } else {
            s_raw
        };

        // RFC 6979 guarantees valid r and s, but assert for safety
        debug_assert!(r != U256::ZERO, "RFC 6979 should never produce r = 0");
        debug_assert!(s != U256::ZERO, "RFC 6979 should never produce s = 0");

        Ok(Signature { r, s })
    }
}

// ECDSA Verification (Step 5)
pub fn verify(public_key: &ProjectivePoint, message_hash: &U256, signature: &Signature) -> bool {
    let order = Secp256k1Params::order();

    // Verify r, s are in valid range [1, n-1]
    if signature.r == U256::ZERO || signature.r >= order {
        return false;
    }
    if signature.s == U256::ZERO || signature.s >= order {
        return false;
    }

    // Convert signature components to scalars
    let s_bytes: [u8; 32] = signature.s.to_be_bytes();
    let s_scalar = match Scalar::from_repr(s_bytes.into()).into_option() {
        Some(s) => s,
        None => return false,
    };

    let r_bytes: [u8; 32] = signature.r.to_be_bytes();
    let r_scalar = match Scalar::from_repr(r_bytes.into()).into_option() {
        Some(r) => r,
        None => return false,
    };

    // Convert message hash to scalar
    let h_bytes: [u8; 32] = message_hash.to_be_bytes();
    let h_scalar = match Scalar::from_repr(h_bytes.into()).into_option() {
        Some(h) => h,
        None => return false,
    };

    // Calculate w = s^(-1) mod n
    let w = match s_scalar.invert().into_option() {
        Some(inv) => inv,
        None => return false,
    };

    // Calculate u1 = h * w, u2 = r * w
    let u1 = h_scalar * w;
    let u2 = r_scalar * w;

    // Calculate result = u1*G + u2*PublicKey
    let result_point = (Secp256k1Params::generator() * u1 + *public_key * u2).to_affine();

    // Check if r == result_point.x mod n
    let result_x_coord = result_point.x();
    let mut result_x_bytes = [0u8; 32];
    result_x_bytes.copy_from_slice(&result_x_coord);
    let result_r = U256::from_be_bytes(result_x_bytes);

    let order_nz = NonZero::new(order).unwrap();
    let result_r = result_r % order_nz;

    result_r == signature.r
}
