use super::Level;
use flate2::{Compress, Compression, Decompress, FlushCompress, FlushDecompress};

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

pub fn compress<'a, const T: usize>(
    level: Level,
    bytes: impl AsRef<[u8]>,
    buffer: &'a mut [u8; T],
) -> crate::Result<&'a [u8]> {
    let mut compress = Compress::new(Compression::new(level.into()), false);

    compress
        .compress(bytes.as_ref(), buffer, FlushCompress::Finish)
        .map_err(|_| crate::Error)?;

    let len = compress.total_out();

    Ok(&buffer[..len as usize])
}

pub fn decompress<'a, const T: usize>(
    bytes: impl AsRef<[u8]>,
    buffer: &'a mut [u8; T],
) -> crate::Result<&'a [u8]> {
    let mut decompress = Decompress::new(false);

    decompress
        .decompress(bytes.as_ref(), buffer, FlushDecompress::Finish)
        .map_err(|_| crate::Error)?;

    let len = decompress.total_out();

    Ok(&buffer[..len as usize])
}

#[cfg(feature = "alloc")]
pub fn compress_vec(level: Level, bytes: impl AsRef<[u8]>) -> crate::Result<Vec<u8>> {
    let bytes = bytes.as_ref();

    let mut buffer = Vec::with_capacity(bytes.len() * 2);

    let mut compress = Compress::new(Compression::new(level.into()), false);

    compress
        .compress_vec(bytes, &mut buffer, FlushCompress::Finish)
        .map_err(|_| crate::Error)?;

    Ok(buffer)
}

#[cfg(feature = "alloc")]
pub fn decompress_vec(bytes: impl AsRef<[u8]>) -> crate::Result<Vec<u8>> {
    let bytes = bytes.as_ref();

    let mut buffer = Vec::with_capacity(bytes.len() * 2);

    let mut decompress = Decompress::new(false);

    decompress
        .decompress_vec(bytes, &mut buffer, FlushDecompress::Finish)
        .map_err(|_| crate::Error)?;

    Ok(buffer)
}
