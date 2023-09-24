use cck_common::size::SIZE_32;
use chacha20poly1305::{ChaCha20Poly1305, XChaCha20Poly1305};

use super::aead::KeyInit;

/// Generate a new xchacha20poly1305 from the given key.
pub fn xchacha20poly1305(key: &[u8; SIZE_32]) -> XChaCha20Poly1305 {
    XChaCha20Poly1305::new_from_slice(key).unwrap()
}

/// Generate a new chacha20poly1305 from the given key.
pub fn chacha20poly1305(key: &[u8; SIZE_32]) -> ChaCha20Poly1305 {
    ChaCha20Poly1305::new_from_slice(key).unwrap()
}
