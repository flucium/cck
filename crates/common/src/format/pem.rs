extern crate alloc;


use crate::{Error, ErrorKind, Result};

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

pub const PEM_LABEL_MESSAGE: &Label = "CCK MESSAGE";

/// Encode a PEM block.
///
/// # Example
/// ```
/// let mut buffer:[u8;1024] = [0u8;1024];
///
/// let pem = pem::encode(pem::PEM_LABEL_PRIVATE_KEY, &private_key, &mut buffer)?;
/// ```
pub fn encode<'a, const T: usize>(
    label: &Label,
    bytes: &'a [u8],
    buffer: &'a mut [u8; T],
) -> Result<&'a str> {
    pem_rfc7468::encode(label, LINE_ENDING, bytes, buffer)
        .map_err(|_| Error::new(ErrorKind::Dummy, String::default()))
}

/// Decode a PEM block.
///
/// # Example
/// ```
/// let mut buffer:[u8;1024] = [0u8;0124];
///
/// let private_key = pem::decode(pem::PEM_LABEL_PRIVATE_KEY, &pem, &mut buffer)?;
/// ```
pub fn decode<'a, const T: usize>(
    label: &Label,
    pem: impl AsRef<[u8]>,
    buffer: &'a mut [u8; T],
) -> Result<&'a [u8]> {
    let (l, bytes) = pem_rfc7468::decode(pem.as_ref(), buffer)
        .map_err(|_| Error::new(ErrorKind::Dummy, String::default()))?;

    if l != label {
        Err(Error::new(ErrorKind::Dummy, String::default()))?
    }

    Ok(bytes)
}
