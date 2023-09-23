// e.g.
// cargo test --package cck-format --test hex --  --nocapture
// cargo test --package cck-format --test hex -- hex_encode --nocapture
// cargo test --package cck-format --test hex -- hex_size_eq --nocapture

#[test]
fn hex_encode() {
    // Hello, world!: [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33]
    const MESSAGE: [u8; 13] = [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33];

    assert_eq!(
        cck_format::hex::encode(&MESSAGE, &mut [0u8; 26]),
        "48656c6c6f2c20776f726c6421"
    );
}

#[test]
fn hex_decode() {
    // Hello, world!: [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33]
    const MESSAGE: [u8; 13] = [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33];

    const HEX: &str = "48656c6c6f2c20776f726c6421";

    assert_eq!(
        cck_format::hex::decode(&HEX, &mut [0u8; MESSAGE.len()]),
        MESSAGE
    );
}

#[test]
fn hex_non_ascii() {
    // non-ascii code
    let BYTES: [u8; 32] = [
        16, 48, 31, 47, 47, 48, 174, 127, 105, 33, 17, 80, 198, 105, 87, 177, 136, 21, 74, 180, 33,
        128, 207, 243, 137, 66, 172, 206, 80, 10, 211, 195,
    ];

    // Encode and get Hex. Decode the obtained Hex and get Bytes.
    // The obtained Bytes are different from the original Bytes.
    // The original Bytes are not ASCII Code.

    // Original bytes: [16, 48, 31, 47, 47, 48, 174, 127, 105, 33, 17, 80, 198, 105, 87, 177, 136, 21, 74, 180, 33, 128, 207, 243, 137, 66, 172, 206, 80, 10, 211, 195]
    // Decoded bytes: [16, 48, 31, 47, 47, 48, 30, 127, 105, 33, 17, 80, 54, 105, 87, 33, 136, 21, 74, 36, 33, 128, 63, 99, 137, 66, 28, 62, 80, 10, 67, 51]
    assert_ne!(
        BYTES,
        cck_format::hex::decode(
            &cck_format::hex::encode(&BYTES, &mut [0u8; 64]),
            &mut [0u8; 32]
        )
    );
}

#[test]
fn hex_size_eq() {
    // Hello, world!: [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33]
    const MESSAGE: [u8; 13] = [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33];

    const HEX: &str = "48656c6c6f2c20776f726c6421";

    assert_eq!(MESSAGE.len() * 2, HEX.len());
}
