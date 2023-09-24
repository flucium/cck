// e.g.
// cargo test --package keyring --test key --  --nocapture
// cargo test --package keyring --test key -- generate_key --nocapture
use keyring::Key;

#[test]
fn generate_key() {
    let private_key = keyring::PrivateKey::generate(keyring::KeyType::Ed25519);

    let public_key = private_key.public_key();

    assert_eq!(private_key.fingerprint(), public_key.fingerprint());

    assert_eq!(private_key.key_type(), public_key.key_type());

    assert_eq!(private_key.expiry(), public_key.expiry());

    assert_eq!(private_key.is_primary(), public_key.is_primary());

    assert_eq!(private_key.signature(), public_key.signature());

    assert_ne!(private_key.as_bytes(), public_key.as_bytes());
}