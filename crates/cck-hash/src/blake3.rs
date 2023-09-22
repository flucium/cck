use cck_common::size::SIZE_32;

pub const BLAKE3_DEFAULT_CONTEXT:&str = "cck-hash-blake3-default-context";

pub const BLAKE3_KEYED_DEFAULT_CONTEXT:&str = "cck-hash-blake3-keyed-default-context";

/// BLAKE3 is a cryptographic hash function that is:
/// 
/// # Example
/// ```
/// let hash = digest(&[],&[]]);
/// ```
pub fn digest(bytes: &[u8], salt: &[u8]) -> [u8; SIZE_32] {
    blake3::Hasher::new()
        .update(bytes)
        .update(salt)
        .finalize()
        .into()
}

/// BLAKE3 derive key with context is a cryptographic hash function and key derivation function that is:
/// 
/// # Example
/// ```
/// let hash = derive_key("context",&[], &[]);
/// ```
pub fn derive_key(context: &str, material: &[u8], salt: &[u8]) -> [u8; SIZE_32] {
    blake3::Hasher::new_derive_key(context)
        .update(material)
        .update(salt)
        .finalize()
        .into()
}

/// BLAKE3 Message Authentication Code is a cryptographic hash function and Message Authentication Code with key that is:
/// 
/// # Example
/// ```
/// let hash = mac(&[0u8;32], &[], &[]);
/// ```
pub fn mac(key: &[u8; SIZE_32], message: &[u8], salt: &[u8]) -> [u8; SIZE_32] {
    blake3::Hasher::new_keyed(key)
        .update(message)
        .update(salt)
        .finalize()
        .into()
}

/// BLAKE3 Extendable Output Function (XOF) is a cryptographic hash function and Extaendable mode that is:
/// 
/// # Example
/// ```
/// let mut buffer:[u8;64] = [0u8;64];
/// 
/// xof_digest(&[], &[], &mut buffer);
/// ```
pub fn xof_digest(bytes: &[u8], salt: &[u8], buffer: &mut [u8]) {
    blake3::Hasher::new()
        .update(bytes)
        .update(salt)
        .finalize_xof()
        .fill(buffer)
}

/// BLAKE3 derive key with context and XOF is a cryptographic hash function and key derivation function and Extendable mode that is:
/// 
/// # Example
/// ```
/// let mut buffer:[u8;64] = [0u8;64];
/// 
/// xof_derive_key("context", &[], &[], &mut buffer);
/// ```
pub fn xof_derive_key(context: &str, material: &[u8], salt: &[u8], buffer: &mut [u8]) {
    blake3::Hasher::new_derive_key(context)
        .update(material)
        .update(salt)
        .finalize_xof()
        .fill(buffer)
}

/// BLAKE3 Message Authentication Code and XOF is a cryptographic hash function and Message Authentication Code with key and Extendable mode that is:
/// 
/// # Example
/// ```
/// let mut buffer:[u8;64] = [0u8;64];
/// 
/// xof_mac(&[0u8;32], &[], &[], &mut buffer);
/// ```
pub fn xof_mac(key: &[u8; SIZE_32], message: &[u8], salt: &[u8], buffer: &mut [u8]) {
    blake3::Hasher::new_keyed(key)
        .update(message)
        .update(salt)
        .finalize_xof()
        .fill(buffer)
}