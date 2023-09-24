use cck_common::size::SIZE_64;

pub fn blake3_digest(key: &[u8]) -> String {
    let mut string = String::new();

    string.push_str("blake3:");

    string.push_str(cck_format::hex::encode(
        &cck_hash::blake3::digest(key, &[]),
        &mut [0u8; SIZE_64],
    ));

    string
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
