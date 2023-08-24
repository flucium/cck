use crate::{Error, Result};
use aead::AeadInPlace;

pub use aead::{arrayvec::ArrayVec, Buffer, KeyInit};

pub(super) fn aead_encrypt(
    aead: impl AeadInPlace,
    nonce: &[u8],
    aad: &[u8],
    buffer: &mut dyn Buffer,
) -> Result<()> {
    aead.encrypt_in_place(nonce.into(), aad, buffer)
        .map_err(|_| Error)
}

pub(super) fn aead_decrypt(
    aead: impl AeadInPlace,
    nonce: &[u8],
    aad: &[u8],
    buffer: &mut dyn Buffer,
) -> Result<()> {
    aead.decrypt_in_place(nonce.into(), aad, buffer)
        .map_err(|_| Error)
}
