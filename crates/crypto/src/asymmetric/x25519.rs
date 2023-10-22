use common::size::SIZE_32;
use crate::rand::*;

/// X25519 Generate Private Key
/// 
/// # Example
/// ```
/// let private_key = gen_private_key();
/// ```
pub fn gen_private_key() -> [u8; SIZE_32] {
    x25519_dalek::StaticSecret::random_from_rng(&mut Rand).to_bytes()
}

/// X25519 Generate Public Key
/// 
/// # Example
/// ```
/// let private_key = gen_private_key();
/// 
/// let public_key = gen_public_key(&private_key);
/// ```
pub fn gen_public_key(private_key: &[u8; SIZE_32]) -> [u8; SIZE_32] {
    *x25519_dalek::PublicKey::from(&x25519_dalek::StaticSecret::from(*private_key)).as_bytes()
}


/// X25519 Diffie-Hellman
/// 
/// You can generate a Symmetric key (or Shared Symmetric key) from one keypair or two keypairs.
/// 
/// # Example
/// ```
/// let private_key = gen_private_key();
/// 
/// let public_key = gen_public_key(&private_key);
/// 
/// let secret = diffie_hellman(&private_key, &public_key);
/// ```
pub fn diffie_hellman(
    private_key: &[u8; SIZE_32],
    their_public_key: &[u8; SIZE_32],
) -> [u8; SIZE_32] {
    *x25519_dalek::StaticSecret::from(*private_key)
        .diffie_hellman(&x25519_dalek::PublicKey::from(*their_public_key))
        .as_bytes()
}