// e.g.
// cargo bench --package cck-hash --bench blake3 -- --exact --nocapture
// cargo bench --package cck-hash --bench blake3 -- blake3_digest --exact --nocapture

#![feature(test)]
extern crate test;
use test::Bencher;

#[bench]
fn blake3_digest(b: &mut Bencher) {
    b.iter(|| cck_hash::blake3::digest(&[0u8; 32], &[]));
}

#[bench]
fn blake3_digest_8192(b: &mut Bencher) {
    b.iter(|| cck_hash::blake3::digest(&[0u8; 8192], &[]));
}

#[bench]
fn blake3_derive_key(b: &mut Bencher) {
    b.iter(|| cck_hash::blake3::derive_key("context", &[0u8; 32], &[]));
}

#[bench]
fn blake3_digest_derive_8192(b: &mut Bencher) {
    b.iter(|| cck_hash::blake3::derive_key("context", &[0u8; 8192], &[]));
}

#[bench]
fn blake3_mac(b: &mut Bencher) {
    b.iter(|| cck_hash::blake3::mac(&[0u8; 32], &[0u8; 32], &[]));
}

#[bench]
fn blake3_digest_mac_8192(b: &mut Bencher) {
    b.iter(|| cck_hash::blake3::mac(&[0u8; 32], &[0u8; 8192], &[]));
}

#[bench]
fn blake3_xof_digest(b: &mut Bencher) {
    let mut buffer: [u8; 128] = [0u8; 128];
    b.iter(|| cck_hash::blake3::xof_digest(&[0u8; 32], &[], &mut buffer));
}

#[bench]
fn blake3_xof_derive_key(b: &mut Bencher) {
    let mut buffer: [u8; 128] = [0u8; 128];
    b.iter(|| cck_hash::blake3::xof_derive_key("context", &[0u8; 32], &[], &mut buffer));
}

#[bench]
fn blake3_xof_mac(b: &mut Bencher) {
    let mut buffer: [u8; 128] = [0u8; 128];

    b.iter(|| cck_hash::blake3::xof_mac(&[0u8; 32], &[0u8; 32], &[], &mut buffer));
}
