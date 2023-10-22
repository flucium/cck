use common::{
    size::{SIZE_16, SIZE_24, SIZE_32},
    Error, ErrorKind, Result,
};

use crate::{
    expiry::Expiry,
    fingerprint::blake3_fingerprint,
    key_type::KeyType,
    rand::{CryptoRng, RngCore},
    string::{decode, encode},
};

/// SharedSecret is an alias for Secret used as the return value of Diffie-Hellman, etc.
pub type SharedSecret = Secret;

pub trait Key {
    fn as_bytes(&self) -> &[u8];

    fn fingerprint(&self) -> &[u8];

    fn signature(&self) -> Option<&[u8]>;

    fn len(&self) -> usize;
}

pub trait AsymmetricKey: Key {
    fn is_primary(&self) -> bool;

    fn key_type(&self) -> &KeyType;

    fn expiry(&self) -> &Expiry;

    fn is_private_key(&self) -> bool;

    /// Create a new key from the given parameters
    fn from(
        primary: bool,
        key_type: KeyType,
        expiry: Expiry,
        key: Vec<u8>,
        fingerprint: Vec<u8>,
        signature: Option<Vec<u8>>,
    ) -> Self;

    /// Encode the key to a string
    fn encode(&self) -> String
    where
        Self: Sized,
    {
        encode(self)
    }

    /// Decode the given string to a key
    fn decode(string: impl Into<String>) -> Result<Self>
    where
        Self: Sized,
    {
        decode(string)
    }
}

/// Secret is the symmetric key used for encryption and decryption.
#[derive(Debug, Clone)]
pub struct Secret {
    bytes: [u8; SIZE_32],
    fingerprint: Vec<u8>,
    signature: Option<Vec<u8>>,
    len: usize,
}

impl Secret {
    /// Generate a new secret key
    ///
    /// # Arguments
    ///
    /// * `rng` - The random number generator
    ///
    /// * `len` - The length of the secret key
    ///
    /// # Errors
    ///
    /// Returns an error if the length is not 16, 24, or 32.
    ///
    /// # Example
    /// ```
    /// let secret = Secret::generate(&mut rand::Rand, size::SIZE_32)?;
    /// ```
    pub fn generate(mut rng: impl CryptoRng + RngCore, len: usize) -> Result<Self> {
        if !matches!(len, SIZE_16 | SIZE_24 | SIZE_32) {
            Err(Error::new(ErrorKind::Dummy, String::default()))?
        }

        let mut bytes = [0; SIZE_32];
        rng.fill_bytes(&mut bytes);

        let fingerprint = blake3_fingerprint(unsafe { &bytes.get_unchecked(..len) });

        let signature: Option<Vec<u8>> = None;

        Ok(Self {
            bytes,
            fingerprint,
            signature,
            len,
        })
    }
}

impl Key for Secret {
    fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn fingerprint(&self) -> &[u8] {
        &self.fingerprint
    }

    fn signature(&self) -> Option<&[u8]> {
        self.signature.as_deref()
    }

    fn len(&self) -> usize {
        self.len
    }
}

/// PrivateKey
///
/// # Example
/// ```
/// let private_key = PrivateKey::generate(KeyType::Ed25519)
/// ```
#[derive(Debug, Clone)]
pub struct PrivateKey {
    pub(super) primary: bool,
    pub(super) key_type: KeyType,
    pub(super) expiry: Expiry,
    pub(super) private_key: Vec<u8>,
    pub(super) public_key: Vec<u8>,
    pub(super) fingerprint: Vec<u8>,
    pub(super) signature: Option<Vec<u8>>,
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
                let private_key = crate::asymmetric::ed25519::gen_private_key();
                let public_key = crate::asymmetric::ed25519::gen_public_key(&private_key);
                (private_key.to_vec(), public_key.to_vec())
            }

            KeyType::X25519 => {
                let private_key = crate::asymmetric::x25519::gen_private_key();
                let public_key = crate::asymmetric::x25519::gen_public_key(&private_key);
                (private_key.to_vec(), public_key.to_vec())
            }
        };

        let fingerprint = blake3_fingerprint(&public_key);

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
    pub fn set_primary(&mut self, is_primary: bool) -> Result<&mut Self> {
        if is_primary && !matches!(self.key_type, KeyType::Ed25519) {
            Err(Error::new(ErrorKind::Dummy, String::default()))?
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
    pub fn derive_key(&self, key_type: KeyType) -> Result<PrivateKey> {
        // Self Generate.
        let mut private_key = match key_type {
            KeyType::Ed25519 => Self::generate(key_type),
            KeyType::X25519 => Self::generate(key_type),
        };

        private_key.set_primary(false)?;

        let signature = match self.key_type {
            KeyType::Ed25519 => crate::asymmetric::ed25519::sign(
                unsafe {
                    self.private_key
                        .get_unchecked(..SIZE_32)
                        .try_into()
                        .unwrap()
                },
                &private_key.public_key,
            )?
            .to_vec(),

            _ => Err(Error::new(ErrorKind::Dummy, String::default()))?,
        };

        // private_key.signature =
        //     Some(cck_format::base64ct::encode(&signature, &mut [0u8; SIZE_64])?.to_owned());
        // private_key.signature =
        //     Some(cck_format::hex::encode(&signature, &mut [0u8; SIZE_64]).to_string());
        private_key.signature = Some(signature);

        Ok(private_key)
    }

    /// Returns the public key
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

impl Key for PrivateKey {
    /// Returns the raw key bytes
    fn as_bytes(&self) -> &[u8] {
        &self.private_key
    }

    /// Returns the fingerprint of the key
    fn fingerprint(&self) -> &[u8] {
        &self.fingerprint
    }

    /// Returns the signature of the key
    fn signature(&self) -> Option<&[u8]> {
        self.signature.as_deref()
    }

    /// Returns the length of the key
    fn len(&self) -> usize {
        self.private_key.len()
    }
}

impl AsymmetricKey for PrivateKey {
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

    /// Returns true if the key is a private key
    ///
    /// Note that this is a private key, so it returns true.
    fn is_private_key(&self) -> bool {
        true
    }

    fn from(
        primary: bool,
        key_type: KeyType,
        expiry: Expiry,
        key: Vec<u8>,
        fingerprint: Vec<u8>,
        signature: Option<Vec<u8>>,
    ) -> Self {
        let public_key = match key_type {
            KeyType::Ed25519 => crate::asymmetric::ed25519::gen_public_key(unsafe {
                key.get_unchecked(..SIZE_32).try_into().unwrap()
            })
            .to_vec(),
            KeyType::X25519 => crate::asymmetric::x25519::gen_public_key(unsafe {
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
            signature: signature,
        }
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
#[derive(Debug, Clone)]
pub struct PublicKey {
    pub(super) primary: bool,
    pub(super) key_type: KeyType,
    pub(super) expiry: Expiry,
    pub(super) public_key: Vec<u8>,
    pub(super) fingerprint: Vec<u8>,
    pub(super) signature: Option<Vec<u8>>,
}

impl Key for PublicKey {
    /// Returns the raw key bytes
    fn as_bytes(&self) -> &[u8] {
        &self.public_key
    }

    /// Returns the fingerprint of the key
    fn fingerprint(&self) -> &[u8] {
        &self.fingerprint
    }

    /// Returns the signature of the key
    fn signature(&self) -> Option<&[u8]> {
        self.signature.as_deref()
    }

    /// Returns the length of the key
    fn len(&self) -> usize {
        self.public_key.len()
    }
}

impl AsymmetricKey for PublicKey {
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

    /// Returns true if the key is a private key
    ///
    /// Note that this is a public key, so it returns false.
    fn is_private_key(&self) -> bool {
        false
    }

    fn from(
        primary: bool,
        key_type: KeyType,
        expiry: Expiry,
        key: Vec<u8>,
        fingerprint: Vec<u8>,
        signature: Option<Vec<u8>>,
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
