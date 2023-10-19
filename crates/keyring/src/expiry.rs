/// Expiry date of a key.
///
/// The expiry date is represented as a tuple of 8 bytes.
///
/// All 0 means it is valid forever.
///
/// Default is all 0.
///
/// - *The first 4 bytes represent the year.*
///
/// - *The next 2 bytes represent the month.*
///
/// - *The last 2 bytes represent the day.*
///
/// This structure only represents the expiration date and does not provide any functionality such as actual activation or revocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expiry(u8, u8, u8, u8, u8, u8, u8, u8);

impl Expiry {
    /// Creates a new Expiry.
    ///
    /// Default is all 0.
    pub const fn new() -> Self {
        Self(0, 0, 0, 0, 0, 0, 0, 0)
    }

    pub fn from_string(string: impl Into<String>) -> cck_common::Result<Self> {
        let value = string.into();
        let value = value.split('/').collect::<Vec<&str>>();

        if value.len() != 3 {
            Err(cck_common::Error)?
        }

        if !value[0].parse::<usize>().unwrap() <= 9999 {
            Err(cck_common::Error)?
        }

        if !value[1].parse::<usize>().unwrap() <= 12 {
            Err(cck_common::Error)?
        }

        if !value[2].parse::<usize>().unwrap() <= 31 {
            Err(cck_common::Error)?
        }

        let year = value[0]
            .chars()
            .map(|ch| ch.to_digit(10))
            .collect::<Option<Vec<_>>>()
            .unwrap();

        let month = value[1]
            .chars()
            .map(|ch| ch.to_digit(10))
            .collect::<Option<Vec<_>>>()
            .unwrap();

        let day = value[2]
            .chars()
            .map(|ch| ch.to_digit(10))
            .collect::<Option<Vec<_>>>()
            .unwrap();

        let date = (
            year[0] as u8,
            year[1] as u8,
            year[2] as u8,
            year[3] as u8,
            month[0] as u8,
            month[1] as u8,
            day[0] as u8,
            day[1] as u8,
        );

        Ok(Self::from(date))
    }

    /// Returns the expiry year of the key.
    pub fn year(&self) -> (u8, u8, u8, u8) {
        (self.0, self.1, self.2, self.3)
    }

    /// Returns the expiry month of the key.
    pub fn month(&self) -> (u8, u8) {
        (self.4, self.5)
    }

    /// Returns the expiry day of the key.
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

// #[cfg(test)]
// mod tests {
    
//     #[test]
//     fn expiry_default() {
//         use super::Expiry;
//         let expiry = Expiry::default();

//         assert_eq!(expiry.year(), (0, 0, 0, 0));

//         assert_eq!(expiry.month(), (0, 0));

//         assert_eq!(expiry.day(), (0, 0));
//     }

//     #[test]
//     fn expiry_from_pattern1() {
//         use super::Expiry;
//         let expiry = Expiry::from((2, 0, 2, 3, 1, 2, 3, 1));

//         assert_eq!(expiry.year(), (2, 0, 2, 3));

//         assert_eq!(expiry.month(), (1, 2));

//         assert_eq!(expiry.day(), (3, 1));
//     }

//     #[test]
//     fn expiry_from_pattern2() {
//         use super::Expiry;
//         let expiry = Expiry::from((2, 0, 2, 3, 0, 2, 0, 1));

//         assert_eq!(expiry.year(), (2, 0, 2, 3));

//         assert_eq!(expiry.month(), (0, 2));

//         assert_eq!(expiry.day(), (0, 1));
//     }
// }
