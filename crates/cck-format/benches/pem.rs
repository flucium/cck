// e.g.
// cargo bench --package cck-format --bench pem -- --exact --nocapture
// cargo bench --package cck-format --bench pem -- pem_encode --exact --nocapture
// cargo bench --features=alloc --package cck-format --bench pem -- --exact --nocapture

#![feature(test)]
extern crate test;
use test::Bencher;


#[bench]
fn pem_encode(b: &mut Bencher) {
    // ed25519 private key
    const PRIVATE_KEY: [u8; 32] = [
        68, 87, 109, 156, 131, 213, 127, 10, 63, 10, 61, 181, 243, 100, 121, 102, 53, 62, 215, 212,
        67, 223, 238, 9, 34, 39, 44, 10, 51, 2, 56, 96,
    ];

    let mut buffer: [u8; 1024] = [0u8; 1024];

    b.iter(|| {
        cck_format::pem::encode(
            cck_format::pem::PEM_LABEL_PRIVATE_KEY,
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
        cck_format::pem::decode(
            cck_format::pem::PEM_LABEL_PRIVATE_KEY,
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
        cck_format::pem::encode_string(cck_format::pem::PEM_LABEL_PRIVATE_KEY, &PRIVATE_KEY).unwrap();
    });
}

#[cfg(feature = "alloc")]
#[bench]
fn pem_decode_vec(b: &mut Bencher) {
    const PRIVATE_KEY_PEM:&str = "-----BEGIN PRIVATE KEY-----\nRFdtnIPVfwo/Cj2182R5ZjU+19RD3+4JIicsCjMCOGA=\n-----END PRIVATE KEY-----\n";


    b.iter(|| {
        cck_format::pem::decode_vec(
            cck_format::pem::PEM_LABEL_PRIVATE_KEY,
            PRIVATE_KEY_PEM,
        )
        .unwrap();
    });
}