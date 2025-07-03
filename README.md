# Moneda

A cryptographic primitives library implemented in Rust for educational purposes.

## Current Implementation

This library provides implementations of fundamental cryptographic building blocks:

### Core Primitives
- **Finite Field Elements** - Arithmetic operations over prime fields with proper error handling
- **Elliptic Curve Points** - Point addition and scalar multiplication on elliptic curves
- **secp256k1 Curve** - Ethereum and Bitcoin's elliptic curve with proper curve parameters

### ECDSA Digital Signatures
- **Complete ECDSA Implementation** - Sign and verify operations following SEC 1 standards
- **RFC 6979 Deterministic Nonces** - Eliminates nonce reuse vulnerabilities
- **Low-s Normalization (BIP-62)** - Prevents signature malleability attacks
- **Proper Modular Reductions** - Spec-compliant hash and coordinate handling
- **Production-Quality Features** - While maintaining educational clarity

## Architecture

The library is organized into focused modules:

```
src/
├── arithmetic/          # Educational implementations
│   ├── field.rs        # Finite field arithmetic
│   └── point.rs        # Elliptic curve point operations
├── curves/             # Curve parameters and definitions
│   └── secp256k1.rs    # Bitcoin's secp256k1 curve
├── crypto/             # High-level cryptographic operations
│   ├── keys.rs         # Private/public key management
│   ├── ecdsa.rs        # ECDSA sign/verify algorithms
│   ├── rfc6979.rs      # Deterministic nonce generation
│   ├── hash.rs         # Message hashing (SHA-256)
│   └── example.rs      # Usage examples and tests
└── errors.rs           # Comprehensive error handling
```

## Running the Example

```bash
# Run the complete ECDSA example
cargo run --bin ecdsa_example

# Run all tests
cargo test

# Run ECDSA-specific tests
cargo test crypto::example
```

## Educational vs Production

**This implementation prioritizes learning and understanding.** While it includes many production-quality features (RFC 6979, BIP-62, proper error handling), it should not be used in production systems.

### What Makes This Educational
- **Readable Implementation** - Algorithm steps are clearly separated and documented
- **Mathematical Transparency** - Core formulas are visible in the code
- **Modular Design** - Easy to understand each component independently
- **Comprehensive Tests** - Verify correctness and demonstrate usage

### Production-Ready Features Included
- **RFC 6979 Deterministic Nonces** - Eliminates catastrophic nonce reuse
- **BIP-62 Low-s Normalization** - Prevents signature malleability
- **Proper Modular Arithmetic** - Correct handling of curve order vs field prime
- **Robust Error Handling** - No panics in normal operation
- **SEC 1 Compliance** - Follows elliptic curve cryptography standards

## Dependencies

The library uses minimal, well-vetted dependencies:
- **k256** - Only for elliptic curve point multiplication
- **crypto-bigint** - For ECDSA scalar arithmetic
- **sha2** - For SHA-256 message hashing
- **hmac** - For RFC 6979 deterministic nonce generation
- **thiserror** - For structured error handling

## Moving to Production

For production use, consider these RustCrypto ecosystem crates:

### Complete Implementations
- **k256** - Full secp256k1 implementation with optimizations
- **ecdsa** - Production ECDSA with extensive testing
- **signature** - Common traits for digital signatures

### Specialized Components
- **Elliptic Curves**: p256, p384, p521 for other NIST curves
- **Hash Functions**: sha3, blake2, blake3 for alternative hashing
- **Key Derivation**: hkdf, pbkdf2 for key management
- **Constant-Time**: subtle for timing attack resistance

## Security Considerations

**Important**: This implementation is for educational purposes. Production systems should use:
- Constant-time implementations to prevent timing attacks
- Hardware security modules for key storage
- Formal verification for critical components
- Regular security audits and updates

## License

This project is for educational purposes only.
