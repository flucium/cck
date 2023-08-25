use crate::{Result,Error};

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};

use base64ct::Encoding;

pub fn base64_decode<'a, const T: usize>(
    b64: impl AsRef<[u8]>,
    buffer: &'a mut [u8; T],
) -> Result<&'a [u8]> {
    base64ct::Base64::decode(b64, buffer).map_err(|_| Error)
}

pub fn base64_encode<'a, const T: usize>(
    bytes: &'a [u8],
    buffer: &'a mut [u8; T],
) -> Result<&'a str> {
    base64ct::Base64::encode(bytes, buffer).map_err(|_| Error)
}


#[cfg(feature = "alloc")]
pub fn base64_encode_string(bytes: &[u8]) -> String {
    base64ct::Base64::encode_string(bytes)
}


#[cfg(feature = "alloc")]
pub fn base64_decode_vec(b64: impl Into<String>) -> Result<Vec<u8>> {
    base64ct::Base64::decode_vec(&b64.into()).map_err(|_| Error)
}
