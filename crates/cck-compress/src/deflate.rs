use super::Level;
use cck_common::{Error, Result};
use flate2::{Compress, Compression, Decompress, FlushCompress, FlushDecompress};

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Compresses the given message with the given level.
///
/// # Arguments
///
/// * `level` - The level to use.
///
/// * `bytes` - The message to compress.
///
/// * `buffer` - The buffer to use.
///
/// # Example
/// ```
/// let mut buffer = [0; 1024];
///
/// let compressed = cck_compress::compress(cck_compress::Level::One, b"Hello, World!", &mut buffer).unwrap();
/// ```
pub fn compress<'a, const T: usize>(
    level: Level,
    bytes: impl AsRef<[u8]>,
    buffer: &'a mut [u8; T],
) -> Result<&'a [u8]> {
    let mut compress = Compress::new(Compression::new(level.into()), false);

    compress
        .compress(bytes.as_ref(), buffer, FlushCompress::Finish)
        .map_err(|_| Error)?;

    let len = compress.total_out();

    Ok(&buffer[..len as usize])
}

/// Decompresses the given message.
/// 
/// # Arguments
/// 
/// * `bytes` - The message to decompress.
/// 
/// * `buffer` - The buffer to use.
/// 
/// # Example
/// ```
/// let compressed = cck_compress::compress(cck_compress::Level::One, b"Hello, World!", &mut buffer).unwrap();
/// 
/// let mut buffer = [0; 1024];
///
/// let decompressed = cck_compress::decompress(compressed, &mut buffer).unwrap();
/// ```
pub fn decompress<'a, const T: usize>(
    bytes: impl AsRef<[u8]>,
    buffer: &'a mut [u8; T],
) -> Result<&'a [u8]> {
    let mut decompress = Decompress::new(false);

    decompress
        .decompress(bytes.as_ref(), buffer, FlushDecompress::Finish)
        .map_err(|_| Error)?;

    let len = decompress.total_out();

    Ok(&buffer[..len as usize])
}

/// Compresses the given message with the given level.
/// 
/// # Arguments
/// 
/// * `level` - The level to use.
/// 
/// * `bytes` - The message to compress.
/// 
/// # Example
/// ```
/// let compressed = cck_compress::compress_vec(cck_compress::Level::One, b"Hello, World!").unwrap();
/// ```
#[cfg(feature = "alloc")]
pub fn compress_vec(level: Level, bytes: impl AsRef<[u8]>) -> Result<Vec<u8>> {
    let bytes = bytes.as_ref();

    let mut buffer = Vec::with_capacity(bytes.len() * 2);

    let mut compress = Compress::new(Compression::new(level.into()), false);

    compress
        .compress_vec(bytes, &mut buffer, FlushCompress::Finish)
        .map_err(|_| Error)?;

    Ok(buffer)
}

/// Decompresses the given message.
/// 
/// # Arguments
/// 
/// * `bytes` - The message to decompress.
/// 
/// # Example
/// ```
/// let compressed = cck_compress::compress_vec(cck_compress::Level::One, b"Hello, World!").unwrap();
/// 
/// let decompressed = cck_compress::decompress_vec(compressed).unwrap();
/// ```
#[cfg(feature = "alloc")]
pub fn decompress_vec(bytes: impl AsRef<[u8]>) -> Result<Vec<u8>> {
    let bytes = bytes.as_ref();

    let mut buffer = Vec::with_capacity(bytes.len() * 2);

    let mut decompress = Decompress::new(false);

    decompress
        .decompress_vec(bytes, &mut buffer, FlushDecompress::Finish)
        .map_err(|_| Error)?;

    Ok(buffer)
}
