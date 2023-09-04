#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use crate::{Error, Result};
use aead::{Aead, AeadInPlace, Payload};

pub use aead::{arrayvec::ArrayVec, Buffer, KeyInit};

pub(super) fn aead_encrypt_in_place(
    aead: impl AeadInPlace,
    nonce: &[u8],
    aad: &[u8],
    buffer: &mut dyn Buffer,
) -> Result<()> {
    aead.encrypt_in_place(nonce.into(), aad, buffer)
        .map_err(|_| Error)
}

pub(super) fn aead_decrypt_in_place(
    aead: impl AeadInPlace,
    nonce: &[u8],
    aad: &[u8],
    buffer: &mut dyn Buffer,
) -> Result<()> {
    aead.decrypt_in_place(nonce.into(), aad, buffer)
        .map_err(|_| Error)
}

#[cfg(feature = "alloc")]
pub(super) fn aead_encrypt(
    aead: impl AeadInPlace,
    nonce: &[u8],
    aad: &[u8],
    message: &[u8],
) -> Result<Vec<u8>> {
    aead.encrypt(
        nonce.into(),
        Payload {
            aad: aad,
            msg: message,
        },
    )
    .map_err(|_| Error)
}

#[cfg(feature = "alloc")]
pub(super) fn aead_decrypt(
    aead: impl AeadInPlace,
    nonce: &[u8],
    aad: &[u8],
    message: &[u8],
) -> Result<Vec<u8>> {
    aead.decrypt(
        nonce.into(),
        Payload {
            aad: aad,
            msg: message,
        },
    )
    .map_err(|_| Error)
}
