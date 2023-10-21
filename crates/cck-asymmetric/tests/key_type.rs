
#[test]
fn key_type_string() {
    
    let key_type = keyring::KeyType::Ed25519;
    assert_eq!(key_type.to_string(), "Ed25519");

    let key_type = keyring::KeyType::X25519;
    assert_eq!(key_type.to_string(), "X25519");
}

#[test]
fn key_type_from_string(){

    let key_type = keyring::KeyType::from("Ed25519").unwrap();
    assert_eq!(key_type, keyring::KeyType::Ed25519);

    let key_type = keyring::KeyType::from("X25519").unwrap();
    assert_eq!(key_type, keyring::KeyType::X25519);
}

#[test]
fn key_type_from_string_lowercase(){

    let key_type =keyring::KeyType::from("ed25519").unwrap();
    assert_eq!(key_type, keyring::KeyType::Ed25519);

    let key_type = keyring::KeyType::from("x25519").unwrap();
    assert_eq!(key_type,keyring::KeyType::X25519);
}

