#[cfg(target_os = "macos")]
const LINE_ENDING: pem_rfc7468::LineEnding = pem_rfc7468::LineEnding::LF;

#[cfg(target_os = "linux")]
const LINE_ENDING: pem_rfc7468::LineEnding = pem_rfc7468::LineEnding::LF;

#[cfg(target_os = "windows")]
const LINE_ENDING: pem_rfc7468::LineEnding = pem_rfc7468::LineEnding::CRLF;

pub type Label = str;

pub const PEM_LABEL_PRIVATE_KEY: &Label = "PRIVATE KEY";

pub const PEM_LABEL_PUBLIC_KEY: &Label = "PUBLIC KEY";


pub fn pem_encode<'a, const T: usize>(
    label: &Label,
    bytes: &'a [u8],
    buffer: &'a mut [u8; T],
) -> crate::Result<&'a str> {
    pem_rfc7468::encode(label, LINE_ENDING, bytes, buffer).map_err(|_| crate::Error)
}

pub fn pem_decode<'a, const T: usize>(
    label: &Label,
    pem: impl AsRef<[u8]>,
    buffer: &'a mut [u8; T],
) -> crate::Result<&'a [u8]> {
    let (l, bytes) = pem_rfc7468::decode(pem.as_ref(), buffer).map_err(|_| crate::Error)?;

    if l != label {
        Err(crate::Error)?
    }

    Ok(bytes)
}
