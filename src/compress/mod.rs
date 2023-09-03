pub mod deflate;

#[repr(u32)]
#[derive(Debug)]
pub enum Level {
    Any(u32),
    Fast = 1,
    Normal = 6,
    Best = 9,
}

impl Level {
    pub(super) fn to_u32(&self) -> u32 {
        match self {
            Self::Any(n) => *n,
            Self::Fast => 1,
            Self::Normal => 6,
            Self::Best => 9,
        }
    }
}

impl core::default::Default for Level {
    fn default() -> Self {
        Self::Normal
    }
}
