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

impl From<(u8, u8, u8, u8, u8, u8, u8, u8)> for Expiry {
    fn from(value: (u8, u8, u8, u8, u8, u8, u8, u8)) -> Self {
        Self(
            value.0, value.1, value.2, value.3, value.4, value.5, value.6, value.7,
        )
    }
}

impl Default for Expiry {
    fn default() -> Self {
        Self(0, 0, 0, 0, 0, 0, 0, 0)
    }
}

impl ToString for Expiry {
    fn to_string(&self) -> String {
        format!(
            "{}{}{}{}/{}{}/{}{}",
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7,
        )
        .to_string()
    }
}

// impl Display for Expiry {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_fmt(format_args!(
//             "{}{}{}{}/{}{}/{}{}",
//             self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7,
//         ))
//     }
// }
