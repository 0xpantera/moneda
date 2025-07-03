use crate::curves::secp256k1::Secp256k1Params;
use crate::errors::EcdsaError;
use k256::elliptic_curve::bigint::{Encoding, U256};
use k256::elliptic_curve::{Field, PrimeField};
use k256::{ProjectivePoint, Scalar};
use rand_core::OsRng;

#[derive(Debug, Clone)]
pub struct PrivateKey {
    scalar: Scalar,
}

#[derive(Debug, Clone)]
pub struct PublicKey {
    point: ProjectivePoint,
}

impl PrivateKey {
    // Step 1: Pick a private key (random scalar)
    pub fn generate() -> Self {
        Self {
            scalar: Scalar::random(&mut OsRng),
        }
    }

    // Alternative: create from specific value (for testing)
    pub fn from_bytes(bytes: &[u8; 32]) -> Result<Self, EcdsaError> {
        let scalar = Scalar::from_repr((*bytes).into())
            .into_option()
            .ok_or(EcdsaError::InvalidPrivateKey)?;
        Ok(Self { scalar })
    }

    // Step 2: Generate public key using private key (privkey * G)
    pub fn public_key(&self) -> PublicKey {
        // Public key = private_key * G (using k256 like ecpy)
        let point = Secp256k1Params::generator() * self.scalar;
        PublicKey { point }
    }

    // Convert to U256 for ECDSA calculations
    pub fn as_u256(&self) -> U256 {
        let bytes = self.scalar.to_bytes();
        U256::from_be_bytes(bytes.into())
    }

    // Get the inner scalar
    pub fn as_scalar(&self) -> &Scalar {
        &self.scalar
    }
}

impl PublicKey {
    pub fn as_point(&self) -> &ProjectivePoint {
        &self.point
    }
}
