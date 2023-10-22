
/// Use BLAKE3 to generate fingerprint.
///
/// Returns the fingerprint of the key.
pub fn blake3_fingerprint(key: &[u8]) -> Vec<u8> {
    crate::hash::blake3::digest(key, &[]).to_vec()
}
