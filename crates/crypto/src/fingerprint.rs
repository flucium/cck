use common::size::SIZE_32;

/// Use BLAKE3 to generate fingerprint.
///
/// Returns the fingerprint of the key.
pub fn blake3_fingerprint(key: &[u8]) -> [u8;SIZE_32] {
    crate::hash::blake3::digest(key, &[])
}
