// e.g.
// cargo bench --package cck --bench format -- --exact --nocapture
// cargo bench --package cck --bench format -- base64_encode --exact --nocapture
// cargo bench --features=alloc --package cck --bench format -- --exact --nocapture
// cargo bench --features=alloc --package cck --bench format -- base64_encode_string --exact --nocapture

#![feature(test)]
extern crate test;
use test::Bencher;

#[bench]
fn base64_encode(b: &mut Bencher) {
    // hello: [104, 101, 108, 108, 111]
    const BYTES: [u8; 5] = [104, 101, 108, 108, 111];

    let mut buffer: [u8; 1024] = [0u8; 1024];

    b.iter(|| {
        cck::format::base64_encode(&BYTES, &mut buffer).unwrap();
    });
}

#[bench]
fn base64_decode(b: &mut Bencher) {
    const B64: &str = "aGVsbG8=";

    let mut buffer: [u8; 10] = [0u8; 10];

    b.iter(|| {
        cck::format::base64_decode(B64, &mut buffer).unwrap();
    });
}

#[cfg(feature = "alloc")]
#[bench]
fn base64_encode_string(b: &mut Bencher) {
    // hello: [104, 101, 108, 108, 111]
    const BYTES: [u8; 5] = [104, 101, 108, 108, 111];

    b.iter(|| {
        cck::format::base64_encode_string(&BYTES);
    });
}

#[cfg(feature = "alloc")]
#[bench]
fn base64_decode_vec(b: &mut Bencher) {
    const B64: &str = "aGVsbG8=";

    b.iter(|| {
        cck::format::base64_decode_vec(B64).unwrap();
    });
}

#[bench]
fn pem_encode(b: &mut Bencher) {
    // ed25519 private key
    const PRIVATE_KEY: [u8; 32] = [
        68, 87, 109, 156, 131, 213, 127, 10, 63, 10, 61, 181, 243, 100, 121, 102, 53, 62, 215, 212,
        67, 223, 238, 9, 34, 39, 44, 10, 51, 2, 56, 96,
    ];

    let mut buffer: [u8; 1024] = [0u8; 1024];

    b.iter(|| {
        cck::format::pem_encode(
            cck::format::PEM_LABEL_PRIVATE_KEY,
            &PRIVATE_KEY,
            &mut buffer,
        )
        .unwrap();
    });
}

#[bench]
fn pem_decode(b: &mut Bencher) {
    const PRIVATE_KEY_PEM:&str = "-----BEGIN PRIVATE KEY-----\nRFdtnIPVfwo/Cj2182R5ZjU+19RD3+4JIicsCjMCOGA=\n-----END PRIVATE KEY-----\n";

    let mut buffer: [u8; 256] = [0u8; 256];

    b.iter(|| {
        cck::format::pem_decode(
            cck::format::PEM_LABEL_PRIVATE_KEY,
            PRIVATE_KEY_PEM,
            &mut buffer,
        )
        .unwrap();
    });
}

#[cfg(feature = "alloc")]
#[bench]
fn pem_encode_string(b: &mut Bencher) {
    // ed25519 private key
    const PRIVATE_KEY: [u8; 32] = [
        68, 87, 109, 156, 131, 213, 127, 10, 63, 10, 61, 181, 243, 100, 121, 102, 53, 62, 215, 212,
        67, 223, 238, 9, 34, 39, 44, 10, 51, 2, 56, 96,
    ];

    b.iter(|| {
        cck::format::pem_encode_string(cck::format::PEM_LABEL_PRIVATE_KEY, &PRIVATE_KEY).unwrap();
    });
}

#[cfg(feature = "alloc")]
#[bench]
fn pem_decode_vec(b: &mut Bencher) {
    const PRIVATE_KEY_PEM:&str = "-----BEGIN PRIVATE KEY-----\nRFdtnIPVfwo/Cj2182R5ZjU+19RD3+4JIicsCjMCOGA=\n-----END PRIVATE KEY-----\n";


    b.iter(|| {
        cck::format::pem_decode_vec(
            cck::format::PEM_LABEL_PRIVATE_KEY,
            PRIVATE_KEY_PEM,
        )
        .unwrap();
    });
}

#[bench]
fn hex_encode(b: &mut Bencher) {
    // hello: [104, 101, 108, 108, 111]
    const BYTES: [u8; 5] = [104, 101, 108, 108, 111];

    let mut buffer: [u8; 1024] = [0u8; 1024];

    b.iter(|| {
        cck::format::hex_encode(&BYTES, &mut buffer);
    });
}

#[bench]
fn hex_decode(b: &mut Bencher) {
    const HEX: &str = "68656c6c6f";

    let mut buffer: [u8; 10] = [0u8; 10];

    b.iter(|| {
        cck::format::hex_decode(HEX, &mut buffer);
    });
}

#[cfg(feature = "alloc")]
#[bench]
fn hex_encode_string(b: &mut Bencher) {
    // hello: [104, 101, 108, 108, 111]
    b.iter(|| {
        cck::format::hex_encode_string(&[104, 101, 108, 108, 111]);
    });
}

#[cfg(feature = "alloc")]
#[bench]
fn hex_decode_vec(b: &mut Bencher) {
    b.iter(|| {
        cck::format::hex_decode_vec("68656c6c6f");
    });
}
