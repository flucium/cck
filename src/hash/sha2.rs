use super::Digest;
use crate::size::{SIZE_32, SIZE_64};


pub fn sha256_digest(bytes: &[u8], salt: &[u8]) -> [u8; SIZE_32] {
    let mut hasher = sha2::Sha256::new();

    hasher.update(bytes);

    hasher.update(salt);

    hasher.finalize_reset().into()
}

pub fn sha512_digest(bytes: &[u8], salt: &[u8]) -> [u8; SIZE_64] {
    let mut hasher = sha2::Sha512::new();

    hasher.update(bytes);

    hasher.update(salt);

    hasher.finalize_reset().into()
}

pub fn sha512_256_digest(bytes: &[u8], salt: &[u8]) -> [u8; SIZE_32] {
    let mut hasher = sha2::Sha512_256::new();

    hasher.update(bytes);

    hasher.update(salt);

    hasher.finalize_reset().into()
}
