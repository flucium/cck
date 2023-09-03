// e.g.
// cargo bench --package cck --bench hash -- --exact --nocapture
// cargo bench --package cck --bench hash -- blake3_digest --exact --nocapture
// cargo bench --package cck --bench hash -- sha256_digest --exact --nocapture

#![feature(test)]
extern crate test;
use test::Bencher;

#[bench]
fn blake3_digest(b: &mut Bencher) {
    b.iter(|| cck::hash::blake3_digest(&[0u8; 32], &[]));
}

#[bench]
fn blake3_derive_key(b: &mut Bencher) {
    b.iter(|| cck::hash::blake3_derive_key("context", &[0u8; 32], &[]));
}

#[bench]
fn blake3_mac(b: &mut Bencher) {
    b.iter(|| cck::hash::blake3_mac(&[0u8; 32], &[0u8; 32], &[]));
}

#[bench]
fn blake3_xof_digest(b: &mut Bencher) {
    let mut buffer: [u8; 128] = [0u8; 128];
    b.iter(|| cck::hash::blake3_xof_digest(&[0u8; 32], &[], &mut buffer));
}

#[bench]
fn blake3_xof_derive_key(b: &mut Bencher) {
    let mut buffer: [u8; 128] = [0u8; 128];
    b.iter(|| cck::hash::blake3_xof_derive_key("context", &[0u8; 32], &[], &mut buffer));
}

#[bench]
fn blake3_xof_mac(b: &mut Bencher) {
    let mut buffer: [u8; 128] = [0u8; 128];

    b.iter(|| cck::hash::blake3_xof_mac(&[0u8; 32], &[0u8; 32], &[], &mut buffer));
}

#[bench]
fn sha256_digest(b: &mut Bencher) {
    b.iter(|| cck::hash::sha256_digest(&[0u8; 32], &[]));
}

#[bench]
fn sha512_digest(b: &mut Bencher) {
    b.iter(|| cck::hash::sha512_digest(&[0u8; 32], &[]));
}

#[bench]
fn sha512_256_digest(b: &mut Bencher) {
    b.iter(|| cck::hash::sha512_256_digest(&[0u8; 32], &[]));
}
