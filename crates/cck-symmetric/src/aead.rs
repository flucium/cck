#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "alloc")]
use aead::{Aead, Payload};


use cck_common::{Error, Result};
use aead::AeadInPlace;

pub use aead::{arrayvec::ArrayVec, Buffer, KeyInit};

pub fn aead_encrypt_in_place(
    aead: &impl AeadInPlace,
    nonce: &[u8],
    associated_data: &[u8],
    buffer: &mut dyn Buffer,
) -> Result<()> {
    aead.encrypt_in_place(nonce.into(), associated_data, buffer)
        .map_err(|_| Error)
}

pub fn aead_decrypt_in_place(
    aead: &impl AeadInPlace,
    nonce: &[u8],
    associated_data: &[u8],
    buffer: &mut dyn Buffer,
) -> Result<()> {
    aead.decrypt_in_place(nonce.into(), associated_data, buffer)
        .map_err(|_| Error)
}

#[cfg(feature = "alloc")]
pub fn aead_encrypt(
    aead: &impl AeadInPlace,
    nonce: &[u8],
    associated_data: &[u8],
    message: &[u8],
) -> Result<Vec<u8>> {
    aead.encrypt(
        nonce.into(),
        Payload {
            aad: associated_data,
            msg: message,
        },
    )
    .map_err(|_| Error)
}

#[cfg(feature = "alloc")]
pub fn aead_decrypt(
    aead: &impl AeadInPlace,
    nonce: &[u8],
    associated_data: &[u8],
    message: &[u8],
) -> Result<Vec<u8>> {
    aead.decrypt(
        nonce.into(),
        Payload {
            aad: associated_data,
            msg: message,
        },
    )
    .map_err(|_| Error)
}
