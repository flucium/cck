use cck_common::size::SIZE_32;
use std::default::Default;

use crate::{Expiry, KeyType};

pub trait Key {
    fn is_primary(&self) -> bool;

    fn key_type(&self) -> &KeyType;

    fn expiry(&self) -> &Expiry;

    fn as_bytes(&self) -> &[u8];

    fn signature(&self) -> Option<&[u8]>;

    fn fingerprint(&self) -> String;

    fn is_private_key(&self) -> bool;
}

pub struct PublicKey {
    primary: bool,
    key_type: KeyType,
    expiry: Expiry,
    public_key: Vec<u8>,
    signature: Option<Vec<u8>>,
}

impl Key for PublicKey {
    fn is_primary(&self) -> bool {
        self.primary
    }

    fn key_type(&self) -> &KeyType {
        &self.key_type
    }

    fn expiry(&self) -> &Expiry {
        &self.expiry
    }

    fn as_bytes(&self) -> &[u8] {
        &self.public_key
    }

    fn signature(&self) -> Option<&[u8]> {
        Some(self.signature.as_ref()?)
    }

    fn fingerprint(&self) -> String {
        let public_key = self.public_key.as_ref();

        crate::fingerprint::blake3_digest(public_key)
    }

    fn is_private_key(&self) -> bool {
        false
    }
}

pub struct PrivateKey {
    primary: bool,
    key_type: KeyType,
    expiry: Expiry,
    private_key: Vec<u8>,
    public_key: Vec<u8>,
    signature: Option<Vec<u8>>,
}

impl Key for PrivateKey {
    fn is_primary(&self) -> bool {
        self.primary
    }

    fn key_type(&self) -> &KeyType {
        &self.key_type
    }

    fn expiry(&self) -> &Expiry {
        &self.expiry
    }

    fn as_bytes(&self) -> &[u8] {
        &self.private_key
    }

    fn signature(&self) -> Option<&[u8]> {
        Some(self.signature.as_ref()?)
    }

    fn fingerprint(&self) -> String {
        let public_key = self.public_key.as_ref();

        crate::fingerprint::blake3_digest(public_key)
    }

    fn is_private_key(&self) -> bool {
        true
    }
}

impl PrivateKey {
    pub fn generate(key_type: KeyType) -> Self {
        let (private_key, public_key) = match key_type {
            KeyType::Ed25519 => {
                let private_key = cck_asymmetric::ed25519::gen_private_key();
                let public_key = cck_asymmetric::ed25519::gen_public_key(&private_key);
                (private_key.to_vec(), public_key.to_vec())
            }

            KeyType::X25519 => {
                let private_key = cck_asymmetric::x25519::gen_private_key();
                let public_key = cck_asymmetric::x25519::gen_public_key(&private_key);
                (private_key.to_vec(), public_key.to_vec())
            }
        };

        Self {
            primary: false,
            key_type: key_type,
            expiry: Expiry::default(),
            private_key: private_key,
            public_key: public_key,
            signature: None,
        }
    }

    pub fn set_primary(&mut self, is_primary: bool) -> cck_common::Result<&mut Self> {
        if is_primary && !matches!(self.key_type, KeyType::Ed25519) {
            Err(cck_common::Error)?
        }

        self.primary = is_primary;
        Ok(self)
    }

    pub fn set_expiry(&mut self, expiry: Expiry) -> &mut Self {
        self.expiry = expiry;

        self
    }

    pub fn derive_key(&self, key_type: KeyType) -> cck_common::Result<PrivateKey> {
        let mut private_key = match key_type {
            KeyType::Ed25519 => Self::generate(key_type),
            KeyType::X25519 => Self::generate(key_type),
        };

        private_key.set_primary(false)?;

        let signature = match self.key_type {
            KeyType::Ed25519 => cck_asymmetric::ed25519::sign(
                unsafe {
                    self.private_key
                        .get_unchecked(..SIZE_32)
                        .try_into()
                        .unwrap()
                },
                &private_key.public_key,
            )?
            .to_vec(),

            _ => Err(cck_common::Error)?,
        };

        private_key.signature = Some(signature);

        Ok(private_key)
    }

    pub fn public_key(&self) -> PublicKey {
        PublicKey {
            primary: self.primary,
            key_type: self.key_type.clone(),
            expiry: self.expiry.clone(),
            public_key: self.public_key.clone(),
            signature: self.signature.clone(),
        }
    }
}

// #[derive(Debug)]
// pub struct Key {
//     master: bool,
//     key_type: KeyType,
//     expiry: Expiry,
//     private_key: Vec<u8>,
//     public_key: Vec<u8>,
//     signature: Option<Vec<u8>>,
// }

// impl Key {
//     pub fn generate(key_type: KeyType) -> Self {
//         let (private_key, public_key) = match key_type {
//             KeyType::Ed25519 => {
//                 let private_key = cck_asymmetric::ed25519::gen_private_key();
//                 let public_key = cck_asymmetric::ed25519::gen_public_key(&private_key);
//                 (private_key.to_vec(), public_key.to_vec())
//             }

//             KeyType::X25519 => {
//                 let private_key = cck_asymmetric::x25519::gen_private_key();
//                 let public_key = cck_asymmetric::x25519::gen_public_key(&private_key);
//                 (private_key.to_vec(), public_key.to_vec())
//             }
//         };

//         Self {
//             master: false,
//             key_type: key_type,
//             expiry: Expiry::default(),
//             private_key: private_key,
//             public_key: public_key,
//             signature: None,
//         }
//     }

//     pub fn derive_key(&self, key_type: KeyType) -> cck_common::Result<Key> {
//         let mut key = match key_type {
//             KeyType::Ed25519 => Self::generate(key_type),
//             KeyType::X25519 => Self::generate(key_type),
//         };

//         key.set_master(false).unwrap();

//         let signature = match self.key_type {
//             KeyType::Ed25519 => cck_asymmetric::ed25519::sign(
//                 unsafe {
//                     self.private_key
//                         .get_unchecked(..SIZE_32)
//                         .try_into()
//                         .unwrap()
//                 },
//                 &key.public_key,
//             )?
//             .to_vec(),

//             _ => Err(cck_common::Error)?,
//         };

//         key.signature = Some(signature);

//         Ok(key)
//     }

//     pub fn set_expiry(&mut self, expiry: Expiry) -> &mut Self {
//         self.expiry = expiry;

//         self
//     }

//     pub fn set_master(&mut self, is_master: bool) -> cck_common::Result<&mut Self> {
//         if is_master && !matches!(self.key_type, KeyType::Ed25519) {
//             Err(cck_common::Error)?
//         }

//         self.master = is_master;
//         Ok(self)
//     }

//     pub fn is_master(&self) -> bool {
//         self.master
//     }

//     pub fn key_type(&self) -> &KeyType {
//         &self.key_type
//     }

//     pub fn expiry(&self) -> &Expiry {
//         &self.expiry
//     }

//     pub fn private_key(&self) -> &[u8] {
//         &self.private_key
//     }

//     pub fn public_key(&self) -> &[u8] {
//         &self.public_key
//     }

//     pub fn signature(&self) -> Option<&[u8]> {
//         Some(self.signature.as_ref()?)
//     }

//     pub fn fingerprint(&self) -> String {
//         let public_key = self.public_key.as_ref();

//         crate::fingerprint::blake3_digest(public_key)
//     }
// }
