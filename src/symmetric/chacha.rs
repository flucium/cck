#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "alloc")]
use super::aead::{aead_decrypt,aead_encrypt};

use chacha20poly1305::{ChaCha20Poly1305, XChaCha20Poly1305};

use super::aead::{aead_decrypt_in_place, aead_encrypt_in_place, Buffer, KeyInit};

use crate::{
    size::{SIZE_12, SIZE_24, SIZE_32},
    Result,
};

pub fn xchacha20_poly1305_decrypt_in_place(
    key: &[u8; SIZE_32],
    nonce: &[u8; SIZE_24],
    associated_data: &[u8],
    buffer: &mut dyn Buffer,
) -> Result<()> {
    aead_decrypt_in_place(
        XChaCha20Poly1305::new_from_slice(key).unwrap(),
        nonce,
        associated_data,
        buffer,
    )
}

pub fn xchacha20_poly1305_encrypt_in_place(
    key: &[u8; SIZE_32],
    nonce: &[u8; SIZE_24],
    associated_data: &[u8],
    buffer: &mut dyn Buffer,
) -> Result<()> {
    aead_encrypt_in_place(
        XChaCha20Poly1305::new_from_slice(key).unwrap(),
        nonce,
        associated_data,
        buffer,
    )
}

pub fn chacha20_poly1305_decrypt_in_place(
    key: &[u8; SIZE_32],
    nonce: &[u8; SIZE_12],
    associated_data: &[u8],
    buffer: &mut dyn Buffer,
) -> Result<()> {
    aead_decrypt_in_place(
        ChaCha20Poly1305::new_from_slice(key).unwrap(),
        nonce,
        associated_data,
        buffer,
    )
}

pub fn chacha20_poly1305_encrypt_in_place(
    key: &[u8; SIZE_32],
    nonce: &[u8; SIZE_12],
    associated_data: &[u8],
    buffer: &mut dyn Buffer,
) -> Result<()> {
    aead_encrypt_in_place(
        ChaCha20Poly1305::new_from_slice(key).unwrap(),
        nonce,
        associated_data,
        buffer,
    )
}

#[cfg(feature = "alloc")]
pub fn xchacha20_poly1305_decrypt(
    key: &[u8; SIZE_32],
    nonce: &[u8; SIZE_24],
    associated_data: &[u8],
    message: &[u8],
) -> Result<Vec<u8>> {
    aead_decrypt(
        XChaCha20Poly1305::new_from_slice(key).unwrap(),
        nonce,
        associated_data,
        message,
    )
}

#[cfg(feature = "alloc")]
pub fn xchacha20_poly1305_encrypt(
    key: &[u8; SIZE_32],
    nonce: &[u8; SIZE_24],
    associated_data: &[u8],
    message: &[u8],
) -> Result<Vec<u8>> {
    aead_encrypt(
        XChaCha20Poly1305::new_from_slice(key).unwrap(),
        nonce,
        associated_data,
        message,
    )
}

#[cfg(feature = "alloc")]
pub fn chacha20_poly1305_decrypt(
    key: &[u8; SIZE_32],
    nonce: &[u8; SIZE_12],
    associated_data: &[u8],
    message: &[u8],
) -> Result<Vec<u8>> {
    aead_decrypt(
        ChaCha20Poly1305::new_from_slice(key).unwrap(),
        nonce,
        associated_data,
        message,
    )
}

#[cfg(feature = "alloc")]
pub fn chacha20_poly1305_encrypt(
    key: &[u8; SIZE_32],
    nonce: &[u8; SIZE_12],
    associated_data: &[u8],
    message: &[u8],
) -> Result<Vec<u8>> {
    aead_encrypt(
        ChaCha20Poly1305::new_from_slice(key).unwrap(),
        nonce,
        associated_data,
        message,
    )
}
