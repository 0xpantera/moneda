# Moneda

A toy implementation of cryptographic primitives in Rust for educational purposes.

## Current Implementation

This library currently provides basic implementations of:

- **Finite Field Elements** - Arithmetic operations over prime fields
- **Elliptic Curve Points** - Point addition and scalar multiplication on elliptic curves

⚠️ **Warning**: This is a learning implementation and is **NOT** suitable for production use. It lacks security hardening and may be vulnerable to timing attacks.

## Making This Production Ready

To convert this toy implementation into a production-ready cryptographic library, the following changes would be needed:

### Security Hardening
- Replace variable-time operations with constant-time implementations using [`subtle`](https://crates.io/crates/subtle)
- Switch from `num-bigint` to [`crypto-bigint`](https://github.com/RustCrypto/crypto-bigint) for cryptographically secure big integer operations
- Implement proper error handling (replace panics with `Result` types)

### RustCrypto Ecosystem Integration

For specific cryptographic applications, consider these RustCrypto crates:

- **Elliptic Curves**: [`k256`](https://github.com/RustCrypto/elliptic-curves/tree/master/k256) (secp256k1), [`p256`](https://github.com/RustCrypto/elliptic-curves/tree/master/p256), [`p384`](https://github.com/RustCrypto/elliptic-curves/tree/master/p384)
- **Digital Signatures**: [`ecdsa`](https://github.com/RustCrypto/signatures/tree/master/ecdsa), [`ed25519`](https://github.com/RustCrypto/signatures/tree/master/ed25519)
- **Hash Functions**: [`sha2`](https://github.com/RustCrypto/hashes/tree/master/sha2), [`sha3`](https://github.com/RustCrypto/hashes/tree/master/sha3)
- **Traits**: [`signature`](https://github.com/RustCrypto/traits/tree/master/signature), [`digest`](https://github.com/RustCrypto/traits/tree/master/digest)

## Running Tests

```bash
cargo test
```

## License

This project is for educational purposes only.