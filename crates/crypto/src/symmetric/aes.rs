use aes_gcm::{Aes128Gcm, Aes256Gcm};

use common::size::{SIZE_16, SIZE_24, SIZE_32};

type Aes192Gcm = aes_gcm::AesGcm<aes_gcm::aes::Aes192, aes_gcm::aes::cipher::consts::U12>;

use super::aead::KeyInit;

/// Generate a new aes_128_gcm from the given key.
pub fn aes_128_gcm(key: &[u8; SIZE_16]) -> Aes128Gcm {
    Aes128Gcm::new_from_slice(key).unwrap()
}

/// Generate a new aes_192_gcm from the given key.
pub fn aes_192_gcm(key: &[u8; SIZE_24]) -> Aes192Gcm {
    Aes192Gcm::new_from_slice(key).unwrap()
}

/// Generate a new aes_256_gcm from the given key.
pub fn aes_256_gcm(key: &[u8; SIZE_32]) -> Aes256Gcm {
    Aes256Gcm::new_from_slice(key).unwrap()
}
