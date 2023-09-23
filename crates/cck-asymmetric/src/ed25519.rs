use cck_common::{
    size::{SIZE_32, SIZE_64},
    Error, Result,
};

use cck_rand::*;

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

pub fn gen_private_key() -> [u8; SIZE_32] {
    ed25519_dalek::SigningKey::generate(&mut Rand).to_bytes()
}

pub fn gen_public_key(private_key: &[u8; SIZE_32]) -> [u8; SIZE_32] {
    ed25519_dalek::SigningKey::from_bytes(private_key)
        .verifying_key()
        .to_bytes()
}

pub fn sign(private_key: &[u8; SIZE_32], message: &[u8]) -> Result<[u8; SIZE_64]> {
    let signature = SigningKey::from_bytes(private_key)
        .try_sign(message)
        .map_err(|_| Error)?;

    Ok(signature.to_bytes())
}

pub fn verify(public_key: &[u8; SIZE_32], message: &[u8], signature: &[u8; SIZE_64]) -> Result<()> {
    VerifyingKey::from_bytes(public_key)
        .map_err(|_| Error)?
        .verify(message, &Signature::from_bytes(signature))
        .map_err(|_| Error)
}
