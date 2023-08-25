mod size;
pub mod rand;
pub mod symmetric;
pub mod asymmetric;
pub mod hash;
pub mod format;

pub type Result<T> = core::result::Result<T,Error>;

#[derive(Debug,Clone)]
pub struct Error;