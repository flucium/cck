// e.g.
// cargo test --package cck-format --test base64ct --  --nocapture
// cargo test --package cck-format --test base64ct -- base64ct_encode --nocapture

#[test]
fn base64ct_encode() {
    // Hello, world!: [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33]
    const MESSAGE: [u8; 13] = [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33];

    assert_eq!(
        cck_format::base64ct::encode(&MESSAGE, &mut [0u8; 26]).unwrap(),
        "SGVsbG8sIHdvcmxkIQ=="
    );
}

#[test]
fn base64ct_decode() {
    // Hello, world!: [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33]
    const MESSAGE: [u8; 13] = [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33];

    const BASE64: &str = "SGVsbG8sIHdvcmxkIQ==";

    assert_eq!(
        cck_format::base64ct::decode(&BASE64, &mut [0u8; MESSAGE.len()]).unwrap(),
        MESSAGE
    );
}
