use std::fmt::Display;


#[derive(Debug)]
pub struct Expiry(u8, u8, u8, u8, u8, u8, u8, u8);

impl Expiry {
    pub fn year(&self) -> (u8, u8, u8, u8) {
        (self.0, self.1, self.2, self.3)
    }

    pub fn month(&self) -> (u8, u8) {
        (self.4, self.5)
    }

    pub fn day(&self) -> (u8, u8) {
        (self.6, self.7)
    }
}

impl Default for Expiry {
    fn default() -> Self {
        Self(0, 0, 0, 0, 0, 0, 0, 0)
    }
}

impl Display for Expiry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}{}{}{}/{}{}/{}{}",
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7,
        ))
    }
}
