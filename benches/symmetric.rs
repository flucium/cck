// e.g.
// cargo bench --package cck --bench symmetric -- --exact --nocapture
// cargo bench --package cck --bench symmetric -- chacha20_poly1305_encrypt_in_place --exact --nocapture

#![feature(test)]
extern crate test;
use test::Bencher;

use cck::symmetric::*;

#[bench]
fn chacha20_poly1305_encrypt_in_place(b: &mut Bencher) {
    const KEY: [u8; 32] = [
        57, 175, 86, 245, 102, 95, 243, 137, 254, 235, 187, 7, 87, 88, 175, 190, 102, 82, 188, 163,
        54, 51, 85, 130, 172, 177, 0, 252, 130, 32, 174, 81,
    ];

    const NONCE: [u8; 12] = [237, 234, 221, 165, 161, 138, 43, 236, 203, 229, 63, 230];

    b.iter(|| {
        let mut message: cck::symmetric::ArrayVec<u8, 256> = cck::symmetric::ArrayVec::new();

        // hello: [104, 101, 108, 108, 111]
        message
            .extend_from_slice(&[104, 101, 108, 108, 111])
            .unwrap();

        cck::symmetric::chacha20_poly1305_encrypt_in_place(&KEY, &NONCE, &[], &mut message).unwrap()
    })
}

#[bench]
fn chacha20_poly1305_decrypt_in_place(b: &mut Bencher) {
    const KEY: [u8; 32] = [
        57, 175, 86, 245, 102, 95, 243, 137, 254, 235, 187, 7, 87, 88, 175, 190, 102, 82, 188, 163,
        54, 51, 85, 130, 172, 177, 0, 252, 130, 32, 174, 81,
    ];

    const NONCE: [u8; 12] = [237, 234, 221, 165, 161, 138, 43, 236, 203, 229, 63, 230];

    b.iter(move || {
        let mut message: cck::symmetric::ArrayVec<u8, 256> = cck::symmetric::ArrayVec::new();

        message
            .extend_from_slice(&[
                30, 117, 55, 72, 38, 100, 128, 57, 130, 159, 56, 119, 83, 106, 118, 249, 117, 18,
                77, 97, 79,
            ])
            .unwrap();

        cck::symmetric::chacha20_poly1305_decrypt_in_place(&KEY, &NONCE, &[], &mut message).unwrap()
    })
}
