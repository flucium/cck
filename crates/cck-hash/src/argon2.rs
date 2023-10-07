use cck_common::size::SIZE_32;

pub enum Argorithm {
    Argon2i,
    Argon2d,
    Argon2id,
}

pub enum Version {
    V0x10,
    V0x13,
}

pub enum Params {
    Default,
    Custom {
        m_cost: u32,
        t_cost: u32,
        p_cost: u32,
    },
}

// #[cfg(feature = "alloc")]
// extern crate alloc;

// #[cfg(feature = "alloc")]
// use alloc::vec::Vec;

/// Argon2i
///
/// Argon2i is the best choice for password hashing and password-based key derivation.
///
/// Version: 0x13 (19)
///
/// Memory: 4096 KiB, Iterations: 3, Parallelism: 1
///
/// Output length: 32 bytes
pub fn argon2i() -> argon2::Argon2<'static> {
    argon2::Argon2::new(
        argon2::Algorithm::Argon2i,
        argon2::Version::V0x13,
        argon2::Params::default(),
    )
}

/// Argon2d
///
/// Argon2d is faster and uses data-depending memory access, which makes it highly resistant against GPU cracking attacks and suitable for applications with no threats from side-channel timing attacks (eg. cryptocurrencies).
///
/// Version: 0x13 (19)
///
/// Memory: 4096 KiB, Iterations: 3, Parallelism: 1
///
/// Output length: 32 bytes
pub fn argon2d() -> argon2::Argon2<'static> {
    argon2::Argon2::new(
        argon2::Algorithm::Argon2d,
        argon2::Version::V0x13,
        argon2::Params::default(),
    )
}

/// Argon2id
///
/// Argon2id is a hybrid of Argon2i and Argon2d, using a combination of data-depending and data-independent memory accesses, which gives some of Argon2i's resistance to side-channel cache timing attacks and much of Argon2d's resistance to GPU cracking attacks.
///
/// Version: 0x13 (19)
///
/// Memory: 4096 KiB, Iterations: 3, Parallelism: 1
///
/// Output length: 32 bytes
pub fn argon2id() -> argon2::Argon2<'static> {
    argon2::Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        argon2::Params::default(),
    )
}

/// Argon2 Custom
pub fn argon2_custom(
    argorithm: Argorithm,
    version: Version,
    params: Params,
) -> cck_common::Result<argon2::Argon2<'static>> {
    let params = match params {
        Params::Default => argon2::Params::default(),
        Params::Custom {
            m_cost,
            t_cost,
            p_cost,
        } => argon2::Params::new(m_cost, t_cost, p_cost, Some(SIZE_32))
            .map_err(|_| cck_common::Error)?,
    };

    Ok(argon2::Argon2::new(
        match argorithm {
            Argorithm::Argon2i => argon2::Algorithm::Argon2i,
            Argorithm::Argon2d => argon2::Algorithm::Argon2d,
            Argorithm::Argon2id => argon2::Algorithm::Argon2id,
        },
        match version {
            Version::V0x10 => argon2::Version::V0x10,
            Version::V0x13 => argon2::Version::V0x13,
        },
        params,
    ))
}

/// Digest
///
/// Digests the given password with the given salt.
///
/// # Arguments
///
/// * `argon2` - The Argon2 instance to use.
///
/// * `password` - The password to digest.
///
/// * `salt` - The salt to use.
///
/// # Example
/// ```
/// let argon2 = argon2id();
///
/// let password = b"password"; // any length
///
/// let salt = b"salt"; // 8 bytes recommended
///
/// let digest = cck_hash::digest(&argon2, password, salt);
/// ```
pub fn digest(
    argon2: &argon2::Argon2,
    password: &[u8],
    salt: &[u8],
) -> cck_common::Result<[u8; SIZE_32]> {
    let mut output = [0u8; SIZE_32];

    argon2
        .hash_password_into(password, salt, &mut output)
        .map_err(|_| cck_common::Error)?;
    Ok(output)
}
