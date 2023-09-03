use super::Level;
use flate2::{Compress, Compression, Decompress, FlushCompress};

pub fn compress<'a, const T: usize>(
    level: Level,
    bytes: impl AsRef<[u8]>,
    buffer: &'a mut [u8; T],
) -> crate::Result<&'a [u8]> {
    let mut compress = Compress::new(Compression::new(level.to_u32()), false);

    compress
        .compress(bytes.as_ref(), buffer, FlushCompress::Finish)
        .map_err(|_| crate::Error)?;

    let len = compress.total_out();

    Ok(&buffer[..len as usize])
}

pub fn decompress<'a, const T: usize>(
    level: Level,
    bytes: impl AsRef<[u8]>,
    buffer: &'a mut [u8; T],
) -> crate::Result<&'a [u8]> {
    let mut decompress = Decompress::new(false);
    
    decompress
        .decompress(bytes.as_ref(), buffer, flate2::FlushDecompress::Finish)
        .map_err(|_| crate::Error)?;

    let len = decompress.total_out();

    Ok(&buffer[..len as usize])
}
