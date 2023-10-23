use common::{
    size::{SIZE_1024, SIZE_16, SIZE_24, SIZE_32},
    Error, ErrorKind, Result,
};

use crate::{
    rand::gen_12,
    symmetric::{
        aead::{aead_encrypt, KeyInit},
        aes,
    },
    Key, Secret,
};

pub fn encrypt(secret: impl AsRef<Secret>, message: impl AsRef<[u8]>) -> Result<()> {
    let secret = secret.as_ref();

    let message = message.as_ref();

    let nonce = gen_12();

    match secret.len() {
        SIZE_16 => {
            aes::aes_128_gcm(unsafe {
                secret
                    .as_bytes()
                    .get_unchecked(..SIZE_16)
                    .try_into()
                    .unwrap()
            });
        }
        SIZE_24 => {
            aes::aes_192_gcm(unsafe {
                secret
                    .as_bytes()
                    .get_unchecked(..SIZE_24)
                    .try_into()
                    .unwrap()
            });
        }
        SIZE_32 => {
            aes::aes_256_gcm(unsafe {
                secret
                    .as_bytes()
                    .get_unchecked(..SIZE_32)
                    .try_into()
                    .unwrap()
            });
        }
        _ => Err(Error::new(ErrorKind::Dummy, String::default()))?,
    }

    // aead_encrypt_in_place(&aes, nonce, associated_data, buffer);

    Ok(())
}

// pub fn decrypt(){}
