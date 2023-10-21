use keyring::*;

#[test]
fn generate_private_key() {
    let private_key = keyring::PrivateKey::generate(keyring::KeyType::Ed25519);
    
    assert_eq!(private_key.is_primary(), false);
    assert_eq!(private_key.key_type(), &keyring::KeyType::Ed25519);
    assert_eq!(private_key.as_bytes().len(), 32);
    assert_eq!(private_key.public_key().as_bytes().len(), 32);
    assert_eq!(private_key.fingerprint().len(), 32);
    assert_eq!(private_key.signature().is_some(), false);
}

#[test]
fn generate_public_key() {
    let private_key = keyring::PrivateKey::generate(keyring::KeyType::Ed25519);
    let public_key = private_key.public_key();

    assert_eq!(public_key.is_primary(), false);
    assert_eq!(public_key.key_type(), &keyring::KeyType::Ed25519);
    assert_eq!(public_key.as_bytes().len(), 32);
    assert_eq!(public_key.fingerprint().len(), 32);
    assert_eq!(public_key.signature().is_some(), false);
}

#[test]
fn generate_private_key_set_primary() {
    let mut private_key = keyring::PrivateKey::generate(keyring::KeyType::Ed25519);

    assert_eq!(private_key.is_primary(), false);

    private_key.set_primary(true).unwrap();

    assert_eq!(private_key.is_primary(), true);
}

#[test]
fn is_match_fingerprint() {
    let private_key = keyring::PrivateKey::generate(keyring::KeyType::Ed25519);
    let public_key = private_key.public_key();

    assert_eq!(private_key.fingerprint(), public_key.fingerprint());
}
