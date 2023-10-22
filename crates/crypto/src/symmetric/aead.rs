#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "alloc")]
use aead::{Aead, Payload};

use aead::AeadInPlace;

pub use aead::{arrayvec::ArrayVec, Buffer, KeyInit};

use common::{Error, ErrorKind, Result};

/// Encrypts the given message with the given nonce and associated data.
///
/// # Arguments
///
/// * `aead` - The AEAD algorithm to use.
///
/// * `nonce` - The nonce to use.
///
/// * `associated_data` - The associated data to use.
///
/// * `buffer` - The message to encrypt.
pub fn aead_encrypt_in_place(
    aead: &impl AeadInPlace,
    nonce: &[u8],
    associated_data: &[u8],
    buffer: &mut dyn Buffer,
) -> Result<()> {
    aead.encrypt_in_place(nonce.into(), associated_data, buffer)
        .map_err(|_| Error::new(ErrorKind::Dummy, String::default()))
}

/// Decrypts the given message with the given nonce and associated data.
///
/// # Arguments
///
/// * `aead` - The AEAD algorithm to use.
///
/// * `nonce` - The nonce to use.
///
/// * `associated_data` - The associated data to use.
///
/// * `buffer` - The message(cipher) to decrypt.
pub fn aead_decrypt_in_place(
    aead: &impl AeadInPlace,
    nonce: &[u8],
    associated_data: &[u8],
    buffer: &mut dyn Buffer,
) -> Result<()> {
    aead.decrypt_in_place(nonce.into(), associated_data, buffer)
    .map_err(|_| Error::new(ErrorKind::Dummy, String::default()))
}

/// Encrypts the given message with the given nonce and associated data.
///
/// # Arguments
///
/// * `aead` - The AEAD algorithm to use.
///
/// * `nonce` - The nonce to use.
///
/// * `associated_data` - The associated data to use.
///
/// * `message` - The message to encrypt.
///
/// # Returns
///
/// The encrypted message.
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
    .map_err(|_| Error::new(ErrorKind::Dummy, String::default()))
}

/// Decrypts the given message with the given nonce and associated data.
///
/// # Arguments
///
/// * `aead` - The AEAD algorithm to use.
///
/// * `nonce` - The nonce to use.
///
/// * `associated_data` - The associated data to use.
///
/// * `message` - The message(cipher) to decrypt.
///
/// # Returns
///
/// The decrypted message.
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
    .map_err(|_| Error::new(ErrorKind::Dummy, String::default()))
}
