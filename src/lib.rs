pub mod asymmetric;
pub mod compress;
pub mod format;
pub mod hash;
pub mod rand;
mod size;
pub mod symmetric;
pub mod key_ring;
pub type Result<T> = core::result::Result<T, Error>;
#[derive(Debug,Clone)]
pub struct Error;