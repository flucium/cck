// e.g.
// cargo test --package keyring --test string --  --nocapture

use keyring::Key;

#[test]
fn parsing() {
    let private_key = keyring::PrivateKey::generate(keyring::KeyType::Ed25519);

    let priavte_key_string = private_key.to_string();

    assert_eq!(
        private_key,
        keyring::PrivateKey::from_string(priavte_key_string).unwrap()
    );
}
