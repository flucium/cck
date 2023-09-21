// #[cfg(feature = "alloc")]
// extern crate alloc;

// #[cfg(feature = "alloc")]
// use alloc::{str::*, string::String, vec::Vec};

use core::{default::Default, fmt};

use crate::{
    asymmetric::ed25519,
    hash,
    rand::gen_32,
    size::{SIZE_32, SIZE_64},
};

pub struct Key {
    id: [u8; SIZE_32],
    master: bool,
    key_type: KeyType,
    expire: Expire,
    private_key: [u8; SIZE_32],
    public_key: [u8; SIZE_32],
    signature: Option<[u8; SIZE_64]>,
}

impl Key {
    pub fn generate(key_type: KeyType) -> crate::Result<Self> {
        let id = gen_32();

        let (private_key, public_key) = match key_type {
            KeyType::Ed25519 => {
                let private_key = ed25519::gen_private_key();
                let public_key = ed25519::gen_public_key(&private_key);
                (private_key, public_key)
            }
            KeyType::X25519 => Err(crate::Error)?,
        };

        Ok(Self {
            id: id,
            master: true,
            key_type: key_type,
            expire: Expire::new(),
            private_key: private_key,
            public_key: public_key,
            signature: None,
        })
    }

    pub fn set_expire(&mut self, expire: Expire) -> &mut Self {
        self.expire = expire;
        self
    }

    pub fn id(&self) -> [u8; SIZE_32] {
        self.id
    }

    // ToDo!
    // pub fn derive_key(&self, key_type: KeyType) -> Key {
    //     todo!()
    // }

    pub fn is_master(&self) -> bool {
        self.master
    }

    pub fn key_type(&self) -> &KeyType {
        &self.key_type
    }

    pub fn expire(&self) -> &Expire {
        &self.expire
    }

    pub fn private_key(&self) -> &[u8] {
        unsafe { &self.private_key.get_unchecked(..SIZE_32) }
    }

    pub fn public_key(&self) -> &[u8] {
        unsafe { &self.public_key.get_unchecked(..SIZE_32) }
    }

    pub fn signature(&self) -> Option<&[u8]> {
        match &self.signature {
            None => None,
            Some(signature) => Some(unsafe { &signature.get_unchecked(..SIZE_64) }),
        }
    }

    pub fn fingerprint(&self) -> [u8; SIZE_32] {
        hash::blake3_digest(&self.public_key, &[])
    }
}

// impl From<&[u8]> for Key {
//     fn from(bytes: &[u8]) -> Self {
//         //parse
//         todo!()
//     }
// }

// impl From<&str> for Key {
//     fn from(string: &str) -> Self {
//         //parse
//         todo!()
//     }
// }

// #[cfg(feature = "alloc")]
// impl From<String> for Key {
//     fn from(string: String) -> Self {
//         //parse
//         todo!()
//     }
// }

#[derive(Debug)]
pub enum KeyType {
    Ed25519,
    X25519,
}

impl fmt::Display for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyType::Ed25519 => f.write_str("Ed25519"),
            KeyType::X25519 => f.write_str("X25519"),
        }
    }
}

impl Default for KeyType {
    fn default() -> Self {
        Self::Ed25519
    }
}

pub struct Expire((u8, u8, u8, u8, u8, u8, u8, u8));

impl Expire {
    pub const fn new() -> Self {
        Self((0, 0, 0, 0, 0, 0, 0, 0))
    }
}

impl Default for Expire {
    fn default() -> Self {
        Self((0, 0, 0, 0, 0, 0, 0, 0))
    }
}

// fn fingerprint(key: &[u8]) -> [u8; SIZE_32] {
//     hash::blake3_digest(key, &[])
// }
