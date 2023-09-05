// e.g.
// cargo bench --package cck --bench symmetric -- --exact --nocapture
// cargo bench --features=alloc --package cck --bench symmetric -- --exact --nocapture
// cargo bench --package cck --bench symmetric -- chacha20_poly1305_encrypt_in_place --exact --nocapture
// cargo bench --features=alloc --package cck --bench symmetric -- chacha20_poly1305_encrypt --exact --nocapture

#![feature(test)]
extern crate test;
use test::Bencher;

use cck::symmetric::*;

#[bench]
fn chacha20_poly1305_encrypt_in_place(b: &mut Bencher) {
    b.iter(|| {
        let mut message: cck::symmetric::ArrayVec<u8, 256> = cck::symmetric::ArrayVec::new();

        // hello: [104, 101, 108, 108, 111]
        message
            .extend_from_slice(&[104, 101, 108, 108, 111])
            .unwrap();

        cck::symmetric::aead_encrypt_in_place(
            &cck::symmetric::chacha20poly1305(&[0u8; 32]),
            &[0u8; 12],
            &[],
            &mut message,
        )
        .unwrap();
    })
}

#[bench]
fn chacha20_poly1305_decrypt_in_place(b: &mut Bencher) {
    b.iter(|| {
        let mut message: cck::symmetric::ArrayVec<u8, 256> = cck::symmetric::ArrayVec::new();

        message
            .try_extend_from_slice(&[
                247, 98, 139, 210, 58, 220, 233, 98, 52, 11, 111, 214, 160, 164, 209, 254, 115,
                111, 41, 69, 111,
            ])
            .unwrap();

        cck::symmetric::aead_decrypt_in_place(
            &cck::symmetric::chacha20poly1305(&[0u8; 32]),
            &[0u8; 12],
            &[],
            &mut message,
        )
        .unwrap();
    })
}

#[cfg(feature = "alloc")]
#[bench]
fn chacha20_poly1305_encrypt(b: &mut Bencher) {
    b.iter(|| {
        // hello: [104, 101, 108, 108, 111]
        cck::symmetric::aead_encrypt(
            &cck::symmetric::chacha20poly1305(&[0u8; 32]),
            &[0u8; 12],
            &[],
            &[104, 101, 108, 108, 111],
        )
        .unwrap();
    })
}

#[cfg(feature = "alloc")]
#[bench]
fn chacha20_poly1305_decrypt(b: &mut Bencher) {
    b.iter(|| {
        cck::symmetric::aead_decrypt(
            &cck::symmetric::chacha20poly1305(&[0u8; 32]),
            &[0u8; 12],
            &[],
            &[
                247, 98, 139, 210, 58, 220, 233, 98, 52, 11, 111, 214, 160, 164, 209, 254, 115,
                111, 41, 69, 111,
            ],
        )
        .unwrap();
    })
}
