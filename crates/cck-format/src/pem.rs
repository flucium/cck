#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};

use cck_common::{Result,Error};

#[cfg(target_os = "macos")]
const LINE_ENDING: pem_rfc7468::LineEnding = pem_rfc7468::LineEnding::LF;

#[cfg(target_os = "linux")]
const LINE_ENDING: pem_rfc7468::LineEnding = pem_rfc7468::LineEnding::LF;

#[cfg(target_os = "windows")]
const LINE_ENDING: pem_rfc7468::LineEnding = pem_rfc7468::LineEnding::CRLF;

pub type Label = str;

pub const PEM_LABEL_PRIVATE_KEY: &Label = "PRIVATE KEY";

pub const PEM_LABEL_PUBLIC_KEY: &Label = "PUBLIC KEY";

pub const PEM_LABEL_CCK_PRIVATE_KEY: &Label = "CCK PRIVATE KEY";

pub const PEM_LABEL_CCK_PUBLIC_KEY: &Label = "CCK PUBLIC KEY";

pub fn encode<'a, const T: usize>(
    label: &Label,
    bytes: &'a [u8],
    buffer: &'a mut [u8; T],
) -> Result<&'a str> {
    pem_rfc7468::encode(label, LINE_ENDING, bytes, buffer).map_err(|_| Error)
}

pub fn decode<'a, const T: usize>(
    label: &Label,
    pem: impl AsRef<[u8]>,
    buffer: &'a mut [u8; T],
) -> Result<&'a [u8]> {
    let (l, bytes) = pem_rfc7468::decode(pem.as_ref(), buffer).map_err(|_| Error)?;

    if l != label {
        Err(Error)?
    }

    Ok(bytes)
}

#[cfg(feature = "alloc")]
pub fn encode_string(label: &Label, bytes: &[u8]) -> Result<String> {
    pem_rfc7468::encode_string(label, LINE_ENDING, bytes).map_err(|_| Error)
}

#[cfg(feature = "alloc")]
pub fn decode_vec(label: &Label, pem: impl AsRef<[u8]>) -> Result<Vec<u8>> {
    let (l, bytes) = pem_rfc7468::decode_vec(pem.as_ref()).map_err(|_| Error)?;

    if l != label {
        Err(Error)?
    }

    Ok(bytes)
}
