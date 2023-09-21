pub mod size;
pub mod rand;
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct Error;
