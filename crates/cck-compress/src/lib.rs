pub mod deflate;

/// The compression level.
#[repr(u32)]
#[derive(Debug)]
pub enum Level {
    Any(u32),
    Fast = 1,
    Normal = 6,
    Best = 9,
}

impl From<u32> for Level {
    fn from(level: u32) -> Self {
        Self::Any(level)
    }
}

impl Into<u32> for Level {
    fn into(self) -> u32 {
        match self {
            Level::Any(n) => n as u32,
            Level::Fast => 1 as u32,
            Level::Normal => 6 as u32,
            Level::Best => 9 as u32,
        }
    }
}

impl core::default::Default for Level {

    /// Returns the default compression level.
    /// 
    /// Default: `Level::Normal`
    fn default() -> Self {
        Self::Normal
    }
}
