use crate::size::SIZE_32;

use chacha20poly1305::{ChaCha20Poly1305, XChaCha20Poly1305};

use super::aead::KeyInit;

pub fn xchacha20poly1305(key: &[u8; SIZE_32]) -> XChaCha20Poly1305 {
    XChaCha20Poly1305::new_from_slice(key).unwrap()
}

pub fn chacha20poly1305(key: &[u8; SIZE_32]) -> ChaCha20Poly1305 {
    ChaCha20Poly1305::new_from_slice(key).unwrap()
}
