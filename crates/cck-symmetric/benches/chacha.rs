// e.g.
// cargo bench --package cck-symmetric --bench chacha -- --exact --nocapture
// cargo bench --package cck-symmetric --bench chacha -- chacha20poly1305_encrypt_in_place --exact --nocapture
// cargo bench --features=alloc --package cck-symmetric --bench chacha -- --exact --nocapture

#![feature(test)]
extern crate test;
use test::Bencher;

use cck_symmetric::*;


#[bench]
fn xchacha20_poly1305_encrypt_in_place(b: &mut Bencher) {
    b.iter(|| {
        let mut message: cck_symmetric::ArrayVec<u8, 256> = cck_symmetric::ArrayVec::new();

        // hello: [104, 101, 108, 108, 111]
        message
            .extend_from_slice(&[104, 101, 108, 108, 111])
            .unwrap();

        cck_symmetric::aead_encrypt_in_place(
            &cck_symmetric::chacha::xchacha20poly1305(&[0u8; 32]),
            &[0u8; 24],
            &[],
            &mut message,
        )
        .unwrap();
    })
}

#[bench]
fn xchacha20_poly1305_decrypt_in_place(b: &mut Bencher) {
    b.iter(|| {
        let mut message: cck_symmetric::ArrayVec<u8, 256> = cck_symmetric::ArrayVec::new();

        message
            .try_extend_from_slice(&[
                16, 251, 250, 229, 138, 249, 8, 112, 183, 173, 62, 24, 90, 50, 250, 154, 190, 134,
                87, 188, 17,
            ])
            .unwrap();

        cck_symmetric::aead_decrypt_in_place(
            &cck_symmetric::chacha::xchacha20poly1305(&[0u8; 32]),
            &[0u8; 24],
            &[],
            &mut message,
        )
        .unwrap();
    })
}

#[cfg(feature = "alloc")]
#[bench]
fn xchacha20_poly1305_encrypt(b: &mut Bencher) {
    b.iter(|| {
        // hello: [104, 101, 108, 108, 111]
        cck_symmetric::aead_encrypt(
            &cck_symmetric::chacha::xchacha20poly1305(&[0u8; 32]),
            &[0u8; 24],
            &[],
            &[104, 101, 108, 108, 111],
        )
        .unwrap();
    })
}

#[cfg(feature = "alloc")]
#[bench]
fn xchacha20_poly1305_decrypt(b: &mut Bencher) {
    b.iter(|| {
        cck_symmetric::aead_decrypt(
            &cck_symmetric::chacha::xchacha20poly1305(&[0u8; 32]),
            &[0u8; 24],
            &[],
            &[
                16, 251, 250, 229, 138, 249, 8, 112, 183, 173, 62, 24, 90, 50, 250, 154, 190, 134,
                87, 188, 17,
            ],
        )
        .unwrap();
    })
}

#[bench]
fn chacha20_poly1305_encrypt_in_place(b: &mut Bencher) {
    b.iter(|| {
        let mut message: cck_symmetric::ArrayVec<u8, 256> = cck_symmetric::ArrayVec::new();

        // hello: [104, 101, 108, 108, 111]
        message
            .extend_from_slice(&[104, 101, 108, 108, 111])
            .unwrap();

        cck_symmetric::aead_encrypt_in_place(
            &cck_symmetric::chacha::chacha20poly1305(&[0u8; 32]),
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
        let mut message: cck_symmetric::ArrayVec<u8, 256> = cck_symmetric::ArrayVec::new();

        message
            .try_extend_from_slice(&[
                247, 98, 139, 210, 58, 220, 233, 98, 52, 11, 111, 214, 160, 164, 209, 254, 115,
                111, 41, 69, 111,
            ])
            .unwrap();

        cck_symmetric::aead_decrypt_in_place(
            &cck_symmetric::chacha::chacha20poly1305(&[0u8; 32]),
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
        cck_symmetric::aead_encrypt(
            &cck_symmetric::chacha::chacha20poly1305(&[0u8; 32]),
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
        cck_symmetric::aead_decrypt(
            &cck_symmetric::chacha::chacha20poly1305(&[0u8; 32]),
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
