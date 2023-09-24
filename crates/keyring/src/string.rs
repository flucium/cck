use std::str::FromStr;

use cck_common::size::SIZE_64;

use crate::{Expiry, Key, KeyType};

/*
    Primary
    KeyType:Ed25519
    Expiry:2021-01-01T00:00:00Z
    PublicKey:AAAAAAAAAAAAAAAAAA
    Fingerprint:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=
    Signature:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=
*/
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

    let fingerprint = format!("Fingerprint:{}", key.fingerprint());

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

pub fn decode<T: Key>(string: impl Into<String>) -> cck_common::Result<T> {
    let string = string.into();

    let mut lines = string.lines();

    let primary: bool = match lines.next() {
        None => Err(cck_common::Error)?,
        Some(line) => {
            let (key, value) = match line.split_once(':') {
                None => Err(cck_common::Error)?,
                Some((key, value)) => (key, value),
            };

            if key != "Primary" {
                Err(cck_common::Error)?
            }

            match value {
                "true" => true,
                "false" => false,
                _ => Err(cck_common::Error)?,
            }
        }
    };

    let key_type: KeyType = match lines.next() {
        None => Err(cck_common::Error)?,
        Some(line) => {
            let (key, value) = match line.split_once(':') {
                None => Err(cck_common::Error)?,
                Some((key, value)) => (key, value),
            };

            if key != "KeyType" {
                Err(cck_common::Error)?
            }

            match value {
                "Ed25519" => KeyType::Ed25519,
                "X25519" => KeyType::X25519,
                _ => Err(cck_common::Error)?,
            }
        }
    };

    let expiry: Expiry = match lines.next() {
        None => Err(cck_common::Error)?,
        Some(line) => {
            let (key, value) = match line.split_once(':') {
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
                .map(|ch| ch as usize)
                .collect::<Vec<usize>>();

            let month = value[1]
                .chars()
                .map(|ch| ch as usize)
                .collect::<Vec<usize>>();

            let day = value[2]
                .chars()
                .map(|ch| ch as usize)
                .collect::<Vec<usize>>();

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

            Expiry::from(date)
        }
    };

    let key: Vec<u8> = match lines.next() {
        None => Err(cck_common::Error)?,
        Some(line) => {
            let (key, value) = match line.split_once(':') {
                None => Err(cck_common::Error)?,
                Some((key, value)) => (key, value),
            };

            if key != "PrivateKey" && key != "PublicKey" {
                Err(cck_common::Error)?
            }

            cck_format::base64ct::decode(value, &mut [0u8; SIZE_64])?.to_vec()
        }
    };

    // ToDo!
    match lines.next() {
        None => Err(cck_common::Error)?,
        Some(line) => {
            let (key, value) = match line.split_once(':') {
                None => Err(cck_common::Error)?,
                Some((key, value)) => (key, value),
            };

            if key != "Fingerprint" {
                Err(cck_common::Error)?
            }

            value
        }
    };

    let signature: Option<Vec<u8>> = match lines.next() {
        None => None,
        Some(line) => {
            let (key, value) = match line.split_once(':') {
                None => Err(cck_common::Error)?,
                Some((key, value)) => (key, value),
            };

            if key != "Signature" {
                Err(cck_common::Error)?
            }

            if value.len() == 0 {
                None
            } else {
                Some(cck_format::base64ct::decode(value, &mut [0u8; SIZE_64])?.to_vec())
            }
        }
    };

    Ok(T::from(primary, key_type, expiry, key, signature))
}
