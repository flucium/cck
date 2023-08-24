use chacha20poly1305::{ChaCha20Poly1305,XChaCha20Poly1305};

use super::aead::{aead_decrypt, aead_encrypt, Buffer, KeyInit};

use crate::{
    size::{SIZE_24, SIZE_32, SIZE_12},
    Result,
};


pub fn xchacha20_poly1305_decrypt(
    key: &[u8; SIZE_32],
    nonce: &[u8; SIZE_24],
    associated_data: &[u8],
    buffer: &mut dyn Buffer,
) -> Result<()> {
    aead_decrypt(
        XChaCha20Poly1305::new_from_slice(key).unwrap(),
        nonce,
        associated_data,
        buffer,
    )
}

pub fn xchacha20_poly1305_encrypt(
    key: &[u8; SIZE_32],
    nonce: &[u8; SIZE_24],
    associated_data: &[u8],
    buffer: &mut dyn Buffer,
) -> Result<()> {
    aead_encrypt(
        XChaCha20Poly1305::new_from_slice(key).unwrap(),
        nonce,
        associated_data,
        buffer,
    )
}


pub fn chacha20_poly1305_decrypt(
    key: &[u8; SIZE_32],
    nonce: &[u8; SIZE_12],
    associated_data: &[u8],
    buffer: &mut dyn Buffer,
) -> Result<()> {
    aead_decrypt(
        ChaCha20Poly1305::new_from_slice(key).unwrap(),
        nonce,
        associated_data,
        buffer,
    )
}

pub fn chacha20_poly1305_encrypt(
    key: &[u8; SIZE_32],
    nonce: &[u8; SIZE_12],
    associated_data: &[u8],
    buffer: &mut dyn Buffer,
) -> Result<()> {
    aead_encrypt(
        ChaCha20Poly1305::new_from_slice(key).unwrap(),
        nonce,
        associated_data,
        buffer,
    )
}
