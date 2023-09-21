// e.g.
// cargo bench --package cck-format --bench base64ct -- --exact --nocapture
// cargo bench --package cck-format --bench base64ct -- base64ct_encode --exact --nocapture
// cargo bench --features=alloc --package cck-format --bench base64ct -- --exact --nocapture

#![feature(test)]
extern crate test;
use test::Bencher;

#[bench]
fn base64ct_encode(b: &mut Bencher) {
    // hello: [104, 101, 108, 108, 111]
    const BYTES: [u8; 5] = [104, 101, 108, 108, 111];

    let mut buffer: [u8; 1024] = [0u8; 1024];

    b.iter(|| {
        cck_format::base64ct::encode(&BYTES, &mut buffer).unwrap();
    });
}

#[bench]
fn base64ct_decode(b: &mut Bencher) {
    const B64: &str = "aGVsbG8=";

    let mut buffer: [u8; 10] = [0u8; 10];

    b.iter(|| {
        cck_format::base64ct::decode(B64, &mut buffer).unwrap();
    });
}

#[cfg(feature = "alloc")]
#[bench]
fn base64ct_encode_string(b: &mut Bencher) {
    // hello: [104, 101, 108, 108, 111]
    const BYTES: [u8; 5] = [104, 101, 108, 108, 111];

    b.iter(|| {
        cck_format::base64ct::encode_string(&BYTES);
    });
}

#[cfg(feature = "alloc")]
#[bench]
fn base64ct_decode_vec(b: &mut Bencher) {
    const B64: &str = "aGVsbG8=";

    b.iter(|| {
        cck_format::base64ct::decode_vec(B64).unwrap();
    });
}
