pub use digest::Digest;

use crate::size::{SIZE_32, SIZE_64};

pub const BLAKE3_DEFAULT_CONTEXT:&str = "";

pub fn blake3_digest(bytes: &[u8], salt: &[u8]) -> [u8; SIZE_32] {
    blake3::Hasher::new()
        .update(bytes)
        .update(salt)
        .finalize()
        .into()
}

pub fn blake3_derive_key(context: &str, material: &[u8], salt: &[u8]) -> [u8; SIZE_32] {
    blake3::Hasher::new_derive_key(context)
        .update(material)
        .update(salt)
        .finalize()
        .into()
}

pub fn blake3_mac(key: &[u8; SIZE_32], message: &[u8], salt: &[u8]) -> [u8; SIZE_32] {
    blake3::Hasher::new_keyed(key)
        .update(message)
        .update(salt)
        .finalize()
        .into()
}

pub fn blake3_xof_digest(bytes: &[u8], salt: &[u8], buffer: &mut [u8]) {
    blake3::Hasher::new()
        .update(bytes)
        .update(salt)
        .finalize_xof()
        .fill(buffer)
}

pub fn blake3_xof_derive_key(context: &str, material: &[u8], salt: &[u8], buffer: &mut [u8]) {
    blake3::Hasher::new_derive_key(context)
        .update(material)
        .update(salt)
        .finalize_xof()
        .fill(buffer)
}

pub fn blake3_xof_mac(key: &[u8; SIZE_32], message: &[u8], salt: &[u8], buffer: &mut [u8]) {
    blake3::Hasher::new_keyed(key)
        .update(message)
        .update(salt)
        .finalize_xof()
        .fill(buffer)
}

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
