use cck_common::size::SIZE_32;
use cck_rand::*;

pub fn gen_private_key() -> [u8; SIZE_32] {
    x25519_dalek::StaticSecret::random_from_rng(&mut Rand).to_bytes()
}

pub fn gen_public_key(private_key: &[u8; SIZE_32]) -> [u8; SIZE_32] {
    *x25519_dalek::PublicKey::from(&x25519_dalek::StaticSecret::from(*private_key)).as_bytes()
}

pub fn diffie_hellman(
    private_key: &[u8; SIZE_32],
    their_public_key: &[u8; SIZE_32],
) -> [u8; SIZE_32] {
    *x25519_dalek::StaticSecret::from(*private_key)
        .diffie_hellman(&x25519_dalek::PublicKey::from(*their_public_key))
        .as_bytes()
}
