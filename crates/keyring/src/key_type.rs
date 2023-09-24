use std::fmt::Display;

/// KeyType is an enum that represents the type of key.
/// 
/// - *Ed25519 is a type of key that is used for signing and verifying.*
/// 
/// - *X25519 is a type of key that is used for Diffie-Hellman key exchange (key agreement).*
///
/// Only a signing key can be a Master Key.
#[derive(Debug,PartialEq, Eq)]
pub enum KeyType {
    Ed25519,
    X25519,
}

impl KeyType {
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
