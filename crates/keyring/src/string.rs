use std::str::FromStr;

use cck_common::size::SIZE_64;

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
            cck_format::base64ct::encode_string(key.as_bytes())
        )
    } else {
        format!(
            "PublicKey:{}\n",
            cck_format::base64ct::encode_string(key.as_bytes())
        )
    };

    let fingerprint = format!("Fingerprint:{}\n", key.fingerprint());

    let signature = match key.signature() {
        Some(signature) => format!(
            "Signature:{}\n",
            cck_format::base64ct::encode_string(signature)
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

    // ToDo!
    parse_fingerprint(lines.next().unwrap_or_default().to_string())?;

    let signature = parse_signature(lines.next().unwrap_or_default().to_string())?;

    Ok(T::from(primary, key_type, expiry, key, signature))
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

            Ok(Expiry::from(date))
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

fn parse_fingerprint(string: String) -> cck_common::Result<()> {
    match string.is_empty() {
        true => Err(cck_common::Error)?,
        false => {
            let (key, _) = match string.split_once(':') {
                None => Err(cck_common::Error)?,
                Some((key, value)) => (key, value),
            };

            if key != "Fingerprint" {
                Err(cck_common::Error)?
            }

            // value
            Ok(())
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
                    cck_format::base64ct::decode(value, &mut [0u8; SIZE_64])?.to_vec(),
                ))
            }
        }
    }
}
