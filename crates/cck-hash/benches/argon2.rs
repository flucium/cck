// e.g.
// cargo bench --package cck-hash --bench argon2-- --exact --nocapture
// cargo bench --package cck-hash --bench argon2 -- argon2id_digest --exact --nocapture

#![feature(test)]
extern crate test;
use test::Bencher;

#[bench]
fn argon2id_digest(b: &mut Bencher) {
    b.iter(|| {
        cck_hash::argon2::digest(
            &cck_hash::argon2::argon2id(),
            b"password",
            b"saltsaltsaltsalt",
        )
    });
}
