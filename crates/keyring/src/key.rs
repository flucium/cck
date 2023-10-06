use cck_common::size::{SIZE_32, SIZE_64};
use std::default::Default;

use crate::{
    fingerprint,
    string::{decode, encode},
    Expiry, KeyType,
};

/// Key trait for public and private keys
pub trait Key {
    fn is_primary(&self) -> bool;

    fn key_type(&self) -> &KeyType;

    fn expiry(&self) -> &Expiry;

    fn as_bytes(&self) -> &[u8];

    fn fingerprint(&self) -> &str;

    fn signature(&self) -> Option<&str>;

    fn is_private_key(&self) -> bool;

    fn from(
        primary: bool,
        key_type: KeyType,
        expiry: Expiry,
        key: Vec<u8>,
        fingerprint: String,
        signature: Option<impl Into<String>>,
    ) -> Self;

    /// Convert the key to string
    fn from_string(string: impl Into<String>) -> cck_common::Result<Self>
    where
        Self: Sized,
    {
        decode(string)
    }
}

/// PublicKey
///
/// # Example
/// ```
/// let private_key = PrivateKey::generate(KeyType::Ed25519)
///
/// let public_key = private_key.public_key();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicKey {
    pub(super) primary: bool,
    pub(super) key_type: KeyType,
    pub(super) expiry: Expiry,
    pub(super) public_key: Vec<u8>,
    pub(super) fingerprint: String,
    pub(super) signature: Option<String>,
}

impl Key for PublicKey {
    /// Returns true if the key is primary
    fn is_primary(&self) -> bool {
        self.primary
    }

    /// Returns the key type
    fn key_type(&self) -> &KeyType {
        &self.key_type
    }

    /// Returns the expiry of the key
    fn expiry(&self) -> &Expiry {
        &self.expiry
    }

    /// Returns the raw key bytes
    fn as_bytes(&self) -> &[u8] {
        &self.public_key
    }

    /// Returns the fingerprint of the key
    fn fingerprint(&self) -> &str {
        &self.fingerprint
    }

    /// Returns the signature of the key
    fn signature(&self) -> Option<&str> {
        Some(self.signature.as_ref()?)
    }

    /// Returns true if the key is a private key
    ///
    /// Note that this is a public key, so it returns false.
    fn is_private_key(&self) -> bool {
        false
    }

    /// Generates based on types and structures.
    ///
    /// # Example
    /// ```
    /// let public_key = PublicKey::from(true, KeyType::Ed25519, Expiry::default(), vec![0; 32], None);
    /// ```
    fn from(
        primary: bool,
        key_type: KeyType,
        expiry: Expiry,
        key: Vec<u8>,
        fingerprint: String,
        signature: Option<impl Into<String>>,
    ) -> Self {
        Self {
            primary: primary,
            key_type: key_type,
            expiry: expiry,
            public_key: key,
            fingerprint: fingerprint,
            signature: signature.map(|signature| signature.into()),
        }
    }
}

impl ToString for PublicKey {
    /// Convert the public key to string
    ///
    /// Format is:
    ///
    /// *Primary: true*
    ///
    /// *KeyType: Ed25519*
    ///
    /// *Expiry: 2023/01/01*
    ///
    /// *Key: aaaaaaaaaa...*
    ///
    /// *Fingerprint: blake3:aaaaaaaaaa...*
    ///
    /// *Signature: aaaaaaaaaa...*
    ///
    /// # Example
    /// ```
    /// let private_key = PrivateKey::generate(KeyType::Ed25519)
    ///
    /// let string = private_key.to_string();
    /// ```
    fn to_string(&self) -> String {
        encode(self)
    }
}

/// PrivateKey
///
/// # Example
/// ```
/// let private_key = PrivateKey::generate(KeyType::Ed25519)
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivateKey {
    pub(super) primary: bool,
    pub(super) key_type: KeyType,
    pub(super) expiry: Expiry,
    pub(super) private_key: Vec<u8>,
    pub(super) public_key: Vec<u8>,
    pub(super) fingerprint: String,
    pub(super) signature: Option<String>,
}

impl Key for PrivateKey {
    /// Returns true if the key is primary
    fn is_primary(&self) -> bool {
        self.primary
    }

    /// Returns the key type
    fn key_type(&self) -> &KeyType {
        &self.key_type
    }

    /// Returns the expiry of the key
    fn expiry(&self) -> &Expiry {
        &self.expiry
    }

    /// Returns the raw key bytes
    fn as_bytes(&self) -> &[u8] {
        &self.private_key
    }

    /// Returns the fingerprint of the key
    fn fingerprint(&self) -> &str {
        &self.fingerprint
    }

    /// Returns the signature of the key
    fn signature(&self) -> Option<&str> {
        Some(self.signature.as_ref()?)
    }

    /// Returns true if the key is a private key
    fn is_private_key(&self) -> bool {
        true
    }

    /// Generates based on types and structures.
    ///
    /// # Example
    /// ```
    /// let private_key = PrivateKey::from(true, KeyType::Ed25519, Expiry::default(), vec![0; 32], None);
    /// ```
    fn from(
        primary: bool,
        key_type: KeyType,
        expiry: Expiry,
        key: Vec<u8>,
        fingerprint: String,
        signature: Option<impl Into<String>>,
    ) -> Self {
        let public_key = match key_type {
            KeyType::Ed25519 => cck_asymmetric::ed25519::gen_public_key(unsafe {
                key.get_unchecked(..SIZE_32).try_into().unwrap()
            })
            .to_vec(),
            KeyType::X25519 => cck_asymmetric::x25519::gen_public_key(unsafe {
                key.get_unchecked(..SIZE_32).try_into().unwrap()
            })
            .to_vec(),
        };

        Self {
            primary: primary,
            key_type: key_type,
            expiry: expiry,
            private_key: key,
            public_key: public_key,
            fingerprint: fingerprint,
            signature: signature.map(|signature| signature.into()),
        }
    }
}

impl PrivateKey {
    /// Generate a new private key
    ///
    /// # Example
    /// ```
    /// let private_key = PrivateKey::generate(KeyType::Ed25519)
    /// ```
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

        let fingerprint = fingerprint::blake3_digest(&public_key);

        Self {
            primary: false,
            key_type: key_type,
            expiry: Expiry::default(),
            private_key: private_key,
            public_key: public_key,
            fingerprint: fingerprint,
            signature: None,
        }
    }

    /// Set the key as primary
    ///
    /// Can only be set as primary if the key type is a digital signature key such as Ed25519.
    ///
    /// # Example
    /// ```
    /// let private_key = PrivateKey::generate(KeyType::Ed25519)
    ///
    /// let private_key = private_key.set_primary(true);
    /// ```
    pub fn set_primary(&mut self, is_primary: bool) -> cck_common::Result<&mut Self> {
        if is_primary && !matches!(self.key_type, KeyType::Ed25519) {
            Err(cck_common::Error)?
        }

        self.primary = is_primary;
        Ok(self)
    }

    /// Set the expiry of the key
    ///
    /// Default is 0, which means no expiry
    ///
    /// # Example
    /// ```
    /// let private_key = PrivateKey::generate(KeyType::Ed25519)
    ///
    /// let private_key = private_key.set_expiry(Expiry::new(2021, 12, 31));
    /// ```
    pub fn set_expiry(&mut self, expiry: Expiry) -> &mut Self {
        self.expiry = expiry;

        self
    }

    /// Derive a new key from the private key
    ///
    /// It can only be derived from the digital signature key.
    ///
    /// for example, it can be derived from Ed25519, but not from X25519.
    ///
    /// # Example
    /// ```
    /// let private_key = PrivateKey::generate(KeyType::Ed25519)
    ///
    /// let derived_key = private_key.derive_key(KeyType::X25519);
    /// ```
    pub fn derive_key(&self, key_type: KeyType) -> cck_common::Result<PrivateKey> {
        // Self Generate.
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

        // private_key.signature =
        //     Some(cck_format::base64ct::encode(&signature, &mut [0u8; SIZE_64])?.to_owned());
        private_key.signature =
            Some(cck_format::hex::encode(&signature, &mut [0u8; SIZE_64]).to_string());

        Ok(private_key)
    }

    /// Get the public key from the private key
    ///
    /// # Example
    /// ```
    /// let private_key = PrivateKey::generate(KeyType::Ed25519)
    ///
    /// let public_key:PublicKey = private_key.public_key();
    /// ```
    pub fn public_key(&self) -> PublicKey {
        PublicKey {
            primary: self.primary,
            key_type: self.key_type.clone(),
            expiry: self.expiry.clone(),
            public_key: self.public_key.clone(),
            fingerprint: self.fingerprint.clone(),
            signature: self.signature.clone(),
        }
    }
}

impl ToString for PrivateKey {
    /// Convert the private key to string
    ///
    /// Format is:
    ///
    /// *Primary: true*
    ///
    /// *KeyType: Ed25519*
    ///
    /// *Expiry: 2023/01/01*
    ///
    /// *Key: aaaaaaaaaa...*
    ///
    /// *Fingerprint: blake3:aaaaaaaaaa...*
    ///
    /// *Signature: aaaaaaaaaa...*
    ///
    /// # Example
    /// ```
    /// let private_key = PrivateKey::generate(KeyType::Ed25519)
    ///
    /// let string = private_key.to_string();
    /// ```
    fn to_string(&self) -> String {
        encode(self)
    }
}
