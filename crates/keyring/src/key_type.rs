use std::fmt::Display;

/// KeyType is an enum that represents the type of key.
///
/// - *Ed25519 is a type of key that is used for signing and verifying.*
///
/// - *X25519 is a type of key that is used for Diffie-Hellman key exchange (key agreement).*
///
/// Only a signing key can be a Master Key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyType {
    Ed25519,
    X25519,
}

impl KeyType {
    pub fn from(string: impl Into<String>) -> cck_common::Result<Self> {
        let string = string.into();
        match string.as_ref() {
            "Ed25519" => Ok(Self::Ed25519),
            "X25519" => Ok(Self::X25519),
            _ => Err(cck_common::Error),
        }
    }

    // fn as_bytes(&self) -> &[u8] {
    //     match self {
    //         KeyType::Ed25519 => &[69, 100, 50, 53, 53, 49, 57],
    //         KeyType::X25519 => &[88, 50, 53, 53, 49, 57],
    //     }
    // }

    // fn to_bytes(&self) -> Vec<u8> {
    //     match self {
    //         KeyType::Ed25519 => vec![69, 100, 50, 53, 53, 49, 57],
    //         KeyType::X25519 => vec![88, 50, 53, 53, 49, 57],
    //     }
    // }
}

impl Default for KeyType {
    fn default() -> Self {
        Self::Ed25519
    }
}

impl Display for KeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ed25519 => f.write_str("Ed25519"),
            Self::X25519 => f.write_str("X25519"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_type_default() {
        let key_type = KeyType::default();

        assert_eq!(key_type, KeyType::Ed25519);
    }

    #[test]
    fn key_type_ed25519() {
        let key_type = KeyType::Ed25519;

        assert_eq!(key_type, KeyType::Ed25519);
    }

    #[test]
    fn key_type_x25519() {
        let key_type = KeyType::X25519;

        assert_eq!(key_type, KeyType::X25519);
    }

    #[test]
    fn key_type_to_string() {
        assert_eq!(KeyType::default().to_string(), "Ed25519");

        assert_eq!(KeyType::Ed25519.to_string(), "Ed25519");

        assert_eq!(KeyType::X25519.to_string(), "X25519");
    }
}
