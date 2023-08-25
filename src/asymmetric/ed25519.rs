use ed25519_dalek::{Signer, Verifier};

use crate::{
    rand::Rand,
    size::{SIZE_32, SIZE_64},
    Error, Result,
};

pub fn ed25519_gen_keypair() -> ([u8; SIZE_32], [u8; SIZE_32]) {
    let private_key = ed25519_dalek::SigningKey::generate(&mut Rand);
    
    let public_key = ed25519_dalek::VerifyingKey::from(&private_key);

    (private_key.to_bytes(), public_key.to_bytes())
}

pub fn ed25519_gen_public_key(private_key: &[u8; SIZE_32]) -> [u8; SIZE_32] {
    ed25519_dalek::VerifyingKey::from(&ed25519_dalek::SigningKey::from_bytes(private_key))
        .to_bytes()
}

pub fn ed25519_gen_private_key() -> [u8; SIZE_32] {
    ed25519_dalek::SigningKey::generate(&mut Rand).to_bytes()
}

pub fn ed25519_verify(
    public_key: &[u8; SIZE_32],
    message: &[u8],
    signature: &[u8; SIZE_64],
) -> Result<()> {
    ed25519_dalek::VerifyingKey::from_bytes(public_key)
        .map_err(|_| Error)?
        .verify(message, &ed25519_dalek::Signature::from_bytes(signature))
        .map_err(|_| Error)
}

pub fn ed25519_sign(private_key: &[u8; SIZE_32], message: &[u8]) -> Result<[u8; SIZE_64]> {
    let signature = ed25519_dalek::SigningKey::from_bytes(private_key)
        .try_sign(message)
        .map_err(|_| Error)?;

    Ok(signature.to_bytes())
}
