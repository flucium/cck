use cck_common::size::{SIZE_1024, SIZE_64};
use regex::Regex;

const REGEX_EMAIL_SYNTAX: &str = r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct User {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) email: String,
}

impl User {
    pub fn new(name: impl Into<String>, email: impl Into<String>) -> cck_common::Result<Self> {
        let name = name.into();

        let email = email.into();

        // Validate email syntax
        let re = Regex::new(REGEX_EMAIL_SYNTAX).unwrap();

        if !re.is_match(&email) {
            Err(cck_common::Error)?
        }

        // Generate user ID
        let bytes = cck_hash::blake3::digest(
            format!("{} <{}>", name, email).as_bytes(),
            &cck_rand::gen_32(),
        );

        let id = unsafe {
            cck_format::hex::encode(&bytes, &mut [0u8; SIZE_1024]).get_unchecked(..SIZE_64)
        }
        .to_string();

        Ok(Self { id, name, email })
    }

    /// Get the user id
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get the user name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the user email
    pub fn email(&self) -> &str {
        &self.email
    }
}
