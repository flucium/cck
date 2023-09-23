// e.g.
// cargo test --package keyring --test key_type --  --nocapture
// cargo test --package keyring --test key_type -- key_type_default --nocapture


#[test]
fn key_type_default() {
    let key_type = keyring::KeyType::default();

    assert_eq!(key_type, keyring::KeyType::Ed25519);
}

#[test]
fn key_type_ed25519() {
    let key_type = keyring::KeyType::Ed25519;

    assert_eq!(key_type, keyring::KeyType::Ed25519);
}

#[test]
fn key_type_x25519() {
    let key_type = keyring::KeyType::X25519;

    assert_eq!(key_type, keyring::KeyType::X25519);
}

#[test]
fn key_type_to_string() {
    assert_eq!(keyring::KeyType::default().to_string(), "Ed25519");
    
    assert_eq!(keyring::KeyType::Ed25519.to_string(), "Ed25519");

    assert_eq!(keyring::KeyType::X25519.to_string(), "X25519");
}
