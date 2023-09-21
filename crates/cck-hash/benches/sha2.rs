// e.g.
// cargo bench --package cck-hash --bench sha2 -- --exact --nocapture
// cargo bench --package cck-hash --bench sha2 -- sha256_digest --exact --nocapture


#![feature(test)]
extern crate test;
use test::Bencher;

#[bench]
fn sha256_digest(b: &mut Bencher) {
    b.iter(|| cck_hash::sha2::sha256_digest(&[0u8; 32], &[]));
}


#[bench]
fn sha256_digest_8192(b: &mut Bencher) {
    b.iter(|| cck_hash::sha2::sha256_digest(&[0u8; 8192], &[]));
}

#[bench]
fn sha512_digest(b: &mut Bencher) {
    b.iter(|| cck_hash::sha2::sha512_digest(&[0u8; 32], &[]));
}


#[bench]
fn sha512_digest_8192(b: &mut Bencher) {
    b.iter(|| cck_hash::sha2::sha512_digest(&[0u8; 8192], &[]));
}


#[bench]
fn sha512_256_digest(b: &mut Bencher) {
    b.iter(|| cck_hash::sha2::sha512_256_digest(&[0u8; 32], &[]));
}


#[bench]
fn sha512_256_digest_8192(b: &mut Bencher) {
    b.iter(|| cck_hash::sha2::sha512_256_digest(&[0u8; 8192], &[]));
}