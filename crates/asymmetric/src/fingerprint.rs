/// Use BLAKE3 to generate fingerprint.
///
/// Returns the fingerprint of the key.
pub fn blake3_digest(key: &[u8]) -> Vec<u8> {
    cck_hash::blake3::digest(key, &[]).to_vec()
}

// pub fn sha256_digest(key: &[u8]) -> String {
//     let mut string = String::new();

//     string.push_str("blake3:");

//     string.push_str(cck_format::hex::encode(
//         &cck_hash::sha2::sha256_digest(key, &[]),
//         &mut [0u8; SIZE_64],
//     ));

//     string
// }
