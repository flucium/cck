use crate::{Error, ErrorKind, Result};

extern crate alloc;

use alloc::{string::String, vec::Vec};

use base64ct::Encoding;

// #[cfg(feature = "heapless")]
// pub fn decode<'a, const T: usize>(
//     b64: impl AsRef<[u8]>,
//     buffer: &'a mut [u8; T],
// ) -> Result<&'a [u8]> {
//     base64ct::Base64::decode(b64, buffer)
//         .map_err(|_| Error::new(ErrorKind::Dummy, String::default()))
// }

// #[cfg(feature = "heapless")]
// pub fn encode<'a, const T: usize>(bytes: &'a [u8], buffer: &'a mut [u8; T]) -> Result<&'a str> {
//     base64ct::Base64::encode(bytes, buffer)
//         .map_err(|_| Error::new(ErrorKind::Dummy, String::default()))
// }

/// Encode a base64 string.
///
/// # Example
/// ```
/// let base64 = base64ct::encode_string(&bytes);
/// ```
pub fn encode(bytes: &[u8]) -> String {
    base64ct::Base64::encode_string(bytes)
}

/// Decode a base64 string.
///
/// # Example
/// ```
/// let bytes = base64ct::decode_vec(&base64)?;
/// ```
pub fn decode(b64: impl Into<String>) -> Result<Vec<u8>> {
    base64ct::Base64::decode_vec(&b64.into())
        .map_err(|_| Error::new(ErrorKind::Dummy, String::default()))
}
