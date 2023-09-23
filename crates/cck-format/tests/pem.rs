// e.g.
// cargo test --package cck-format --test pem --  --nocapture
// cargo test --package cck-format --test pem -- pem_encode --nocapture

#[test]
fn pem_encode() {
    const LABEL: &cck_format::pem::Label = "MESSAGE";

    const MESSAGE: [u8; 13] = [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33];

    assert_eq!(
        cck_format::pem::encode(LABEL, &MESSAGE, &mut [0u8; 1024]).unwrap(),
        "-----BEGIN MESSAGE-----\nSGVsbG8sIHdvcmxkIQ==\n-----END MESSAGE-----\n"
    );
}

#[test]
fn pem_decode() {
    const LABEL: &cck_format::pem::Label = "MESSAGE";

    const MESSAGE: [u8; 13] = [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33];

    const PEM: &str = "-----BEGIN MESSAGE-----\nSGVsbG8sIHdvcmxkIQ==\n-----END MESSAGE-----\n";

    assert_eq!(
        cck_format::pem::decode(LABEL, &PEM, &mut [0u8; MESSAGE.len()]).unwrap(),
        MESSAGE
    );
}
