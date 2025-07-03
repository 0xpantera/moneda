// In src/errors.rs
use num_bigint::BigInt;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum FieldError {
    #[error("Prime must be > 1, got {0}")]
    InvalidPrime(BigInt),

    #[error("Elements must be in the same field")]
    DifferentFields,
}

#[derive(Error, Debug, PartialEq)]
pub enum PointError {
    #[error("Point ({x}, {y}) is not on the curve")]
    NotOnCurve { x: String, y: String },

    #[error("Invalid point: ({x}, None) is not valid")]
    InvalidXOnly { x: String },

    #[error("Invalid point: (None, {y}) is not valid")]
    InvalidYOnly { y: String },

    #[error("Points not on same curve")]
    DifferentCurves,

    #[error("Field operation failed: {0}")]
    FieldError(#[from] FieldError),
}

#[derive(Error, Debug, PartialEq)]
pub enum EcdsaError {
    #[error("Invalid private key")]
    InvalidPrivateKey,

    #[error("Invalid nonce")]
    InvalidNonce,

    #[error("Invalid hash")]
    InvalidHash,

    #[error("Invalid r value")]
    InvalidR,

    #[error("Invalid modulus")]
    InvalidModulus,

    #[error("Field operation failed: {0}")]
    FieldError(#[from] FieldError),
}
