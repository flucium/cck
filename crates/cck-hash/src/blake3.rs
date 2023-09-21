use cck_common::size::SIZE_32;
pub const BLAKE3_DEFAULT_CONTEXT:&str = "";

pub fn digest(bytes: &[u8], salt: &[u8]) -> [u8; SIZE_32] {
    blake3::Hasher::new()
        .update(bytes)
        .update(salt)
        .finalize()
        .into()
}

pub fn derive_key(context: &str, material: &[u8], salt: &[u8]) -> [u8; SIZE_32] {
    blake3::Hasher::new_derive_key(context)
        .update(material)
        .update(salt)
        .finalize()
        .into()
}

pub fn mac(key: &[u8; SIZE_32], message: &[u8], salt: &[u8]) -> [u8; SIZE_32] {
    blake3::Hasher::new_keyed(key)
        .update(message)
        .update(salt)
        .finalize()
        .into()
}

pub fn xof_digest(bytes: &[u8], salt: &[u8], buffer: &mut [u8]) {
    blake3::Hasher::new()
        .update(bytes)
        .update(salt)
        .finalize_xof()
        .fill(buffer)
}

pub fn xof_derive_key(context: &str, material: &[u8], salt: &[u8], buffer: &mut [u8]) {
    blake3::Hasher::new_derive_key(context)
        .update(material)
        .update(salt)
        .finalize_xof()
        .fill(buffer)
}

pub fn xof_mac(key: &[u8; SIZE_32], message: &[u8], salt: &[u8], buffer: &mut [u8]) {
    blake3::Hasher::new_keyed(key)
        .update(message)
        .update(salt)
        .finalize_xof()
        .fill(buffer)
}