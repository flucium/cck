use common::size::*;

use rand::{Rng, SeedableRng};

use rand_chacha::{rand_core::Error, ChaCha20Rng};

pub use rand::{
    rngs::{OsRng, ThreadRng},
    CryptoRng, RngCore,
};

/// Rand implements RngCore and CryptoRng.
///
/// Internally, ChaCha20Rng.
#[derive(Clone, Copy, Debug, Default)]
pub struct Rand;

impl CryptoRng for Rand {}

impl RngCore for Rand {
    fn next_u32(&mut self) -> u32 {
        let mut buf: [u8; 4] = [0; 4];

        self.fill_bytes(&mut buf);

        u32::from_le_bytes(buf)
    }

    fn next_u64(&mut self) -> u64 {
        let mut buf: [u8; 8] = [0; 8];

        self.fill_bytes(&mut buf);

        u64::from_le_bytes(buf)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.try_fill_bytes(dest).unwrap();
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        dest.copy_from_slice(&gen_32());
        Ok(())
    }
}

/// ChaCha20Rng
///
/// Generates a 32-byte random bytes.
///
/// # Example
/// ```
/// let bytes = xck::rand::gen_32();
///
/// println!("{:?}",bytes);
/// ```
pub fn gen_32() -> [u8; SIZE_32] {
    ChaCha20Rng::from_entropy().gen::<[u8; SIZE_32]>()
}

/// ChaCha20Rng
///
/// Generates a 24-byte random bytes.
///
/// # Example
/// ```
/// let bytes = xck::rand::gen_24();
///
/// println!("{:?}",bytes);
/// ```
pub fn gen_24() -> [u8; SIZE_24] {
    ChaCha20Rng::from_entropy().gen::<[u8; SIZE_24]>()
}

/// ChaCha20Rng
///
/// Generates a 16-byte random bytes.
///
/// # Example
/// ```
/// let bytes = xck::rand::gen_16();
///
/// println!("{:?}",bytes);
/// ```
pub fn gen_16() -> [u8; SIZE_16] {
    ChaCha20Rng::from_entropy().gen::<[u8; SIZE_16]>()
}

/// ChaCha20Rng
///
/// Generates a 12-byte random bytes.
///
/// # Example
/// ```
/// let bytes = xck::rand::gen_12();
///
/// println!("{:?}",bytes);
/// ```
pub fn gen_12() -> [u8; SIZE_12] {
    ChaCha20Rng::from_entropy().gen::<[u8; SIZE_12]>()
}

/// ChaCha20Rng
///
/// Generate a 32-byte random bytes, within ascii code range.
pub fn gen_32_ascii() -> [u8; SIZE_32] {
    let mut buf = [0u8; SIZE_32];

    for i in 0..SIZE_32 {
        buf[i] = gen_range_ascii();
    }

    buf
}

/// ChaCha20Rng
///
/// Generate a 24-byte random bytes, within ascii code range.
pub fn gen_24_ascii() -> [u8; SIZE_24] {
    let mut buf = [0u8; SIZE_24];

    for i in 0..SIZE_24 {
        buf[i] = gen_range_ascii();
    }

    buf
}

/// ChaCha20Rng
///
/// Generate a 16-byte random bytes, within ascii code range.
pub fn gen_16_ascii() -> [u8; SIZE_16] {
    let mut buf = [0u8; SIZE_16];

    for i in 0..SIZE_16 {
        buf[i] = gen_range_ascii();
    }

    buf
}

/// ChaCha20Rng
///
/// Generate a 12-byte random bytes, within ascii code range.
pub fn gen_12_ascii() -> [u8; SIZE_12] {
    let mut buf = [0u8; SIZE_12];

    for i in 0..SIZE_12 {
        buf[i] = gen_range_ascii();
    }

    buf
}

fn gen_range_ascii() -> u8 {
    rand_chacha::ChaCha20Rng::from_entropy().gen_range(32..126)
}
