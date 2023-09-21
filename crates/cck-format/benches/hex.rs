// e.g.
// cargo bench --package cck-format --bench hex -- --exact --nocapture
// cargo bench --package cck-format --bench hex -- hex_encode --exact --nocapture
// cargo bench --features=alloc --package cck-format --bench hex -- --exact --nocapture

#![feature(test)]
extern crate test;
use test::Bencher;

#[bench]
fn hex_encode(b: &mut Bencher) {
    // hello: [104, 101, 108, 108, 111]
    const BYTES: [u8; 5] = [104, 101, 108, 108, 111];

    let mut buffer: [u8; 1024] = [0u8; 1024];

    b.iter(|| {
        cck_format::hex::encode(&BYTES, &mut buffer);
    });
}

#[bench]
fn hex_decode(b: &mut Bencher) {
    const HEX: &str = "68656c6c6f";

    let mut buffer: [u8; 10] = [0u8; 10];

    b.iter(|| {
        cck_format::hex::decode(HEX, &mut buffer);
    });
}

#[cfg(feature = "alloc")]
#[bench]
fn hex_encode_string(b: &mut Bencher) {
    // hello: [104, 101, 108, 108, 111]
    b.iter(|| {
        cck_format::hex::encode_string(&[104, 101, 108, 108, 111]);
    });
}

#[cfg(feature = "alloc")]
#[bench]
fn hex_decode_vec(b: &mut Bencher) {
    b.iter(|| {
        cck_format::hex::decode_vec("68656c6c6f");
    });
}

