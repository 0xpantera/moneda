// Example implementation following the assignment steps exactly

use crate::crypto::ecdsa::verify;
use crate::crypto::hash::hash_message;
use crate::crypto::keys::PrivateKey;
use k256::elliptic_curve::sec1::ToEncodedPoint;

pub fn ecdsa_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== ECDSA Implementation Example ===\n");

    // Step 1: Pick a private key
    println!("Step 1: Pick a private key");
    let private_key = PrivateKey::generate();
    let privkey_u256 = private_key.as_u256();
    println!("Private key: 0x{:064x}\n", privkey_u256);

    // Step 2: Generate the public key using that private key
    println!("Step 2: Generate public key from private key");
    let public_key = private_key.public_key();
    let pub_point = public_key.as_point();
    let pub_affine = pub_point.to_affine();
    let encoded_point = pub_affine.to_encoded_point(false); // uncompressed
    println!("Public key point:");
    println!("  x: 0x{}", hex::encode(encoded_point.x().unwrap()));
    println!("  y: 0x{}\n", hex::encode(encoded_point.y().unwrap()));

    // Step 3: Pick message m and hash it to produce h
    println!("Step 3: Pick message and hash it");
    let message = b"Hello, RareSkills";
    let h = hash_message(message);
    println!("Message: {:?}", std::str::from_utf8(message)?);
    println!("Hash h: 0x{:064x}\n", h);

    // Step 4: Sign m using private key and random nonce k
    println!("Step 4: Sign message with private key");
    let signature = private_key.sign(&h)?;
    println!("Signature:");
    println!("  r: 0x{:064x}", signature.r);
    println!("  s: 0x{:064x}\n", signature.s);

    // Step 5: Verify (r, s, h, PubKey) is valid
    println!("Step 5: Verify signature");
    let is_valid = verify(pub_point, &h, &signature);
    println!("Signature valid: {}\n", is_valid);

    // Summary - produce (r, s, h, PubKey) as requested
    println!("=== Assignment Output ===");
    println!("(r, s, h, PubKey) = (");
    println!("  r = 0x{:064x},", signature.r);
    println!("  s = 0x{:064x},", signature.s);
    println!("  h = 0x{:064x},", h);
    println!("  PubKey = {{");
    println!("    x: 0x{},", hex::encode(encoded_point.x().unwrap()));
    println!("    y: 0x{}", hex::encode(encoded_point.y().unwrap()));
    println!("  }}");
    println!(")");

    // Verify the math worked correctly
    assert!(is_valid, "ECDSA signature verification failed");
    println!("ECDSA implementation successful");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ecdsa_example() {
        ecdsa_example().expect("ECDSA example should work");
    }

    #[test]
    fn test_known_signature() {
        // Test with a known private key for reproducibility
        let privkey_bytes = [1u8; 32]; // Simple test key
        let private_key = PrivateKey::from_bytes(&privkey_bytes).unwrap();
        let public_key = private_key.public_key();

        let message = b"test message";
        let h = hash_message(message);

        let signature = private_key.sign(&h).unwrap();
        let is_valid = verify(public_key.as_point(), &h, &signature);

        assert!(is_valid, "Known signature should be valid");
    }

    #[test]
    fn test_invalid_signature() {
        let private_key = PrivateKey::generate();
        let public_key = private_key.public_key();

        let message1 = b"original message";
        let message2 = b"different message";

        let h1 = hash_message(message1);
        let h2 = hash_message(message2);

        let signature = private_key.sign(&h1).unwrap();

        // Signature for h1 should not verify against h2
        let is_valid = verify(public_key.as_point(), &h2, &signature);
        assert!(
            !is_valid,
            "Signature should be invalid for different message"
        );
    }
}
