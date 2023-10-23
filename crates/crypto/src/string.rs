use std::str::FromStr;

use common::{
    format::base64ct,
    size::{SIZE_128, SIZE_64},
    Error, ErrorKind, Result,
};

use crate::{expiry::Expiry, key::AsymmetricKey, key_type::KeyType};

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
pub fn encode(key: &impl AsymmetricKey) -> String {
    let primary = format!("Primary:{}\n", key.is_primary());

    let key_type = format!("KeyType:{}\n", key.key_type());

    let expiry = format!("Expiry:{}\n", key.expiry().to_string());

    let k = if key.is_private_key() {
        format!(
            "PrivateKey:{}\n",
            base64ct::encode(key.as_bytes())
        )
    } else {
        format!(
            "PublicKey:{}\n",
            base64ct::encode(key.as_bytes())
        )
    };

    let fingerprint = format!(
        "Fingerprint:{}\n",
        base64ct::encode(key.fingerprint())
    );

    let signature = match key.signature() {
        Some(signature) => format!(
            "Signature:{}\n",
            base64ct::encode(signature)
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
pub fn decode<T: AsymmetricKey>(string: impl Into<String>) -> Result<T> {
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

fn parse_primary(string: String) -> Result<bool> {
    match string.is_empty() {
        true => Err(Error::new(ErrorKind::Dummy, String::default()))?,
        false => {
            let (key, value) = match string.split_once(':') {
                None => Err(Error::new(ErrorKind::Dummy, String::default()))?,
                Some((key, value)) => (key, value),
            };

            if key != "Primary" {
                Err(Error::new(ErrorKind::Dummy, String::default()))?
            }

            match value {
                "true" => Ok(true),
                "false" => Ok(false),
                _ => Err(Error::new(ErrorKind::Dummy, String::default()))?,
            }
        }
    }
}

fn parse_key_type(string: String) -> Result<KeyType> {
    match string.is_empty() {
        true => Err(Error::new(ErrorKind::Dummy, String::default()))?,
        false => {
            let (key, value) = match string.split_once(':') {
                None => Err(Error::new(ErrorKind::Dummy, String::default()))?,
                Some((key, value)) => (key, value),
            };

            if key != "KeyType" {
                Err(Error::new(ErrorKind::Dummy, String::default()))?
            }

            match value {
                "Ed25519" => Ok(KeyType::Ed25519),
                "X25519" => Ok(KeyType::X25519),
                _ => Err(Error::new(ErrorKind::Dummy, String::default())),
            }
        }
    }
}

fn parse_expiry(string: String) -> Result<Expiry> {
    match string.is_empty() {
        true => Err(Error::new(ErrorKind::Dummy, String::default()))?,
        false => {
            let (key, value) = match string.split_once(':') {
                None => Err(Error::new(ErrorKind::Dummy, String::default()))?,
                Some((key, value)) => (key, value),
            };

            if key != "Expiry" {
                Err(Error::new(ErrorKind::Dummy, String::default()))?
            }

            Ok(Expiry::from_string(value.to_owned())?)
        }
    }
}

fn parse_key(string: String) -> Result<Vec<u8>> {
    match string.is_empty() {
        true => Err(Error::new(ErrorKind::Dummy, String::default()))?,
        false => {
            let (key, value) = match string.split_once(':') {
                None => Err(Error::new(ErrorKind::Dummy, String::default()))?,
                Some((key, value)) => (key, value),
            };

            if key != "PrivateKey" && key != "PublicKey" {
                Err(Error::new(ErrorKind::Dummy, String::default()))?
            }

            Ok(base64ct::decode(value)?)
        }
    }
}

fn parse_fingerprint(string: String) -> Result<Vec<u8>> {
    match string.is_empty() {
        true => Err(Error::new(ErrorKind::Dummy, String::default()))?,
        false => {
            let (key, value) = match string.split_once(':') {
                None => Err(Error::new(ErrorKind::Dummy, String::default()))?,
                Some((key, value)) => (key, value),
            };

            if key != "Fingerprint" {
                Err(Error::new(ErrorKind::Dummy, String::default()))?
            }

            Ok(base64ct::decode(value)?)
        }
    }
}

fn parse_signature(string: String) -> Result<Option<Vec<u8>>> {
    match string.is_empty() {
        true => Ok(None),
        false => {
            let (key, value) = match string.split_once(':') {
                None => Err(Error::new(ErrorKind::Dummy, String::default()))?,
                Some((key, value)) => (key, value),
            };

            if key != "Signature" {
                Err(Error::new(ErrorKind::Dummy, String::default()))?
            }

            // ToDo!
            if value.len() == 0 {
                Ok(None)
            } else if value == "None" {
                Ok(None)
            } else {
                Ok(Some(
                    base64ct::decode(value)?
                ))
            }
        }
    }
}
