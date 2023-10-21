use std::str::FromStr;

use cck_common::size::{SIZE_128, SIZE_64};

use crate::{Expiry, Key, KeyType};

/// encode a key to a string
///
///
/// Format example:
///
/// *Primary:true*
///
/// *KeyType:Ed25519*
///
/// *Expiry:2023/01/01*
///
/// *PublicKey:AAAAAAAAAAAAAAAAAA*
///
/// *Fingerprint:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=*
///
/// *Signature:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=*
///
pub fn encode(key: &impl Key) -> String {
    let primary = format!("Primary:{}\n", key.is_primary());

    let key_type = format!("KeyType:{}\n", key.key_type());

    let expiry = format!("Expiry:{}\n", key.expiry().to_string());

    let k = if key.is_private_key() {
        format!(
            "PrivateKey:{}\n",
            cck_format::base64ct::encode(key.as_bytes(), &mut [0u8; SIZE_64]).unwrap()
        )
    } else {
        format!(
            "PublicKey:{}\n",
            cck_format::base64ct::encode(key.as_bytes(), &mut [0u8; SIZE_64]).unwrap()
        )
    };

    let fingerprint = format!(
        "Fingerprint:{}\n",
        cck_format::base64ct::encode(key.fingerprint(), &mut [0u8; SIZE_64]).unwrap()
    );

    let signature = match key.signature() {
        Some(signature) => format!(
            "Signature:{}\n",
            cck_format::base64ct::encode(signature, &mut [0u8; SIZE_128]).unwrap()
        ),
        None => String::from_str("Signature:None").unwrap(),
    };

    let mut string = String::new();

    string.push_str(&primary);

    string.push_str(&key_type);

    string.push_str(&expiry);

    string.push_str(&k);

    string.push_str(&fingerprint);

    string.push_str(&signature);

    string.push('\n');

    string
}

/// decode a key from a string
pub fn decode<T: Key>(string: impl Into<String>) -> cck_common::Result<T> {
    let string = string.into();

    let mut lines = string.lines();

    let primary = parse_primary(lines.next().unwrap_or_default().to_string())?;

    let key_type = parse_key_type(lines.next().unwrap_or_default().to_string())?;

    let expiry = parse_expiry(lines.next().unwrap_or_default().to_string())?;

    let key = parse_key(lines.next().unwrap_or_default().to_string())?;

    let fingerprint = parse_fingerprint(lines.next().unwrap_or_default().to_string())?;

    let signature = parse_signature(lines.next().unwrap_or_default().to_string())?;

    Ok(T::from(
        primary,
        key_type,
        expiry,
        key,
        fingerprint,
        signature,
    ))
}

fn parse_primary(string: String) -> cck_common::Result<bool> {
    match string.is_empty() {
        true => Err(cck_common::Error)?,
        false => {
            let (key, value) = match string.split_once(':') {
                None => Err(cck_common::Error)?,
                Some((key, value)) => (key, value),
            };

            if key != "Primary" {
                Err(cck_common::Error)?
            }

            match value {
                "true" => Ok(true),
                "false" => Ok(false),
                _ => Err(cck_common::Error)?,
            }
        }
    }
}

fn parse_key_type(string: String) -> cck_common::Result<KeyType> {
    match string.is_empty() {
        true => Err(cck_common::Error)?,
        false => {
            let (key, value) = match string.split_once(':') {
                None => Err(cck_common::Error)?,
                Some((key, value)) => (key, value),
            };

            if key != "KeyType" {
                Err(cck_common::Error)?
            }

            match value {
                "Ed25519" => Ok(KeyType::Ed25519),
                "X25519" => Ok(KeyType::X25519),
                _ => Err(cck_common::Error),
            }
        }
    }
}

fn parse_expiry(string: String) -> cck_common::Result<Expiry> {
    match string.is_empty() {
        true => Err(cck_common::Error)?,
        false => {
            let (key, value) = match string.split_once(':') {
                None => Err(cck_common::Error)?,
                Some((key, value)) => (key, value),
            };

            if key != "Expiry" {
                Err(cck_common::Error)?
            }

            Ok(Expiry::from_string(value.to_owned())?)
        }
    }
}

fn parse_key(string: String) -> cck_common::Result<Vec<u8>> {
    match string.is_empty() {
        true => Err(cck_common::Error)?,
        false => {
            let (key, value) = match string.split_once(':') {
                None => Err(cck_common::Error)?,
                Some((key, value)) => (key, value),
            };

            if key != "PrivateKey" && key != "PublicKey" {
                Err(cck_common::Error)?
            }

            Ok(cck_format::base64ct::decode(value, &mut [0u8; SIZE_64])?.to_vec())
        }
    }
}

fn parse_fingerprint(string: String) -> cck_common::Result<Vec<u8>> {
    match string.is_empty() {
        true => Err(cck_common::Error)?,
        false => {
            let (key, value) = match string.split_once(':') {
                None => Err(cck_common::Error)?,
                Some((key, value)) => (key, value),
            };

            if key != "Fingerprint" {
                Err(cck_common::Error)?
            }

            // value
            Ok(cck_format::base64ct::decode(value, &mut [0u8; SIZE_64])?.to_vec())
        }
    }
}

fn parse_signature(string: String) -> cck_common::Result<Option<Vec<u8>>> {
    match string.is_empty() {
        true => Ok(None),
        false => {
            let (key, value) = match string.split_once(':') {
                None => Err(cck_common::Error)?,
                Some((key, value)) => (key, value),
            };

            if key != "Signature" {
                Err(cck_common::Error)?
            }

            // ToDo!
            if value.len() == 0 {
                Ok(None)
            } else if value == "None" {
                Ok(None)
            } else {
                Ok(Some(
                    cck_format::base64ct::decode(value, &mut [0u8; SIZE_128])?.to_vec(),
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Key;

    #[test]
    fn encode_private_key() {
        use crate::{Key, KeyType, PrivateKey, PublicKey};

        use super::{decode, encode};

        let private_key = PrivateKey::generate(KeyType::Ed25519);

        let string = encode(&private_key);

        let decoded_private_key = decode(string).unwrap();

        assert_eq!(private_key, decoded_private_key);
    }

    #[test]
    fn encode_public_key() {
        use crate::{Key, KeyType, PrivateKey, PublicKey};

        use super::{decode, encode};

        let private_key = PrivateKey::generate(KeyType::Ed25519);

        let public_key = private_key.public_key();

        let string = encode(&public_key);

        let decoded_public_key = decode(string).unwrap();

        assert_eq!(public_key, decoded_public_key);
    }

    #[test]
    fn parse_primary() {
        use super::parse_primary;

        let string = String::from("Primary:true");

        let primary = parse_primary(string).unwrap();

        assert_eq!(primary, true);

        let string = String::from("Primary:false");

        let primary = parse_primary(string).unwrap();

        assert_eq!(primary, false);
    }

    #[test]
    fn parse_key_type() {
        use super::parse_key_type;

        let string = String::from("KeyType:Ed25519");

        let key_type = parse_key_type(string).unwrap();

        assert_eq!(key_type, crate::KeyType::Ed25519);
    }

    #[test]
    fn parse_expiry() {
        use super::parse_expiry;

        let string = String::from("Expiry:2023/01/01");

        let expiry = parse_expiry(string).unwrap();

        assert_eq!(expiry, crate::Expiry::from_str("2023/01/01").unwrap());
    }

    #[test]
    fn parse_signature() {
        use super::parse_signature;

        let string = String::from("Signature:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=");

        let signature = parse_signature(string).unwrap();

        assert_eq!(signature, Some(vec![0u8; 64]));
    }

    #[test]
    fn parse_signature_none() {
        use super::parse_signature;

        let string = String::from("Signature:None");

        let signature = parse_signature(string).unwrap();

        assert_eq!(signature, None);
    }

    #[test]
    fn parse_fingerprint() {
        use super::parse_fingerprint;

        let string = String::from("Fingerprint:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=");

        let fingerprint = parse_fingerprint(string).unwrap();

        assert_eq!(fingerprint, b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=");
    }
}
