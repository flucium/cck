mod size;
pub mod rand;
pub mod symmetric;
pub mod asymmetric;

pub type Result<T> = core::result::Result<T,Error>;

pub struct Error;