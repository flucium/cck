use cck_common::size::SIZE_32;

use crate::{Expiry, KeyType};

pub struct Certificate {
    expiry: Expiry,
    key_type: KeyType,
    public_key: Vec<u8>,
    fingerprint: Vec<u8>,
}

impl Certificate {
    pub fn new(
        expiry: Expiry,
        key_type: KeyType,
        public_key: Vec<u8>,
        fingerprint: Vec<u8>,
    ) -> Self {
        Self {
            expiry,
            key_type,
            public_key,
            fingerprint,
        }
    }

    pub fn from(string: impl Into<String>) -> cck_common::Result<Self> {
        parse(string)
    }

    pub fn expiry(&self) -> &Expiry {
        &self.expiry
    }

    pub fn key_type(&self) -> &KeyType {
        &self.key_type
    }

    pub fn public_key(&self) -> &[u8] {
        &self.public_key
    }

    pub fn fingerprint(&self) -> &[u8] {
        &self.fingerprint
    }
}

impl ToString for Certificate {
    fn to_string(&self) -> String {
        let expiry = format!("Expiry:{}\n", self.expiry.to_string());

        let key_type = format!("KeyType:{}\n", self.key_type.to_string());

        let public_key = format!(
            "PublicKey:{}\n",
            cck_format::base64ct::encode_string(&self.public_key)
        );

        let fingerprint = format!("Fingerprint:{}",unsafe{String::from_utf8_unchecked(self.fingerprint.clone())});

        let mut string = String::new();

        string.push_str(&expiry);

        string.push_str(&key_type);

        string.push_str(&public_key);

        string.push_str(&fingerprint);
        
        string
    }
}

fn parse(string: impl Into<String>) -> cck_common::Result<Certificate> {
    let string: String = string.into();

    let entries = string.split('\n').collect::<Vec<&str>>();

    if entries.len() != 4 {
        Err(cck_common::Error)?
    }

    let mut expiry: Expiry = Expiry::default();

    let mut key_type: KeyType = KeyType::default();

    let mut public_key: Vec<u8> = Vec::with_capacity(SIZE_32);

    let mut fingerprint: Vec<u8> = Vec::with_capacity(SIZE_32);

    for entry in entries {
        match entry.split_once(':') {
            None => {
                Err(cck_common::Error)?
            }
            Some((key, value)) => match key {
                "Expiry" => {
                    let value = value.split('/').collect::<Vec<&str>>();

                    if value.len() != 3 {
                        Err(cck_common::Error)?
                    }

                    if value[0].parse::<usize>().unwrap() <= 9999 {
                        Err(cck_common::Error)?
                    }

                    if value[1].parse::<usize>().unwrap() <= 12 {
                        Err(cck_common::Error)?
                    }

                    if value[2].parse::<usize>().unwrap() <= 31 {
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

                    expiry = Expiry::from(date);
                }

                "KeyType" => match value {
                    "Ed25519" => {
                        key_type = KeyType::Ed25519;
                    }

                    "X25519" => {
                        key_type = KeyType::X25519;
                    }

                    _ => Err(cck_common::Error)?,
                },

                "PublicKey" => {
                    public_key = cck_format::base64ct::decode_vec(value)?;
                }

                "Fingerprint" => {
                    fingerprint = value.bytes().collect::<Vec<u8>>();
                }

                _ => Err(cck_common::Error)?,
            },
        }
    }

    Ok(Certificate {
        expiry: expiry,
        key_type: key_type,
        public_key: public_key,
        fingerprint: fingerprint,
    })
}
