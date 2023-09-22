// e.g.
// cargo bench --package cck-symmetric --bench aes -- --exact --nocapture
// cargo bench --package cck-symmetric --bench aes -- aes_128_gcm_encrypt_in_place --exact --nocapture
// cargo bench --features=alloc --package cck-symmetric --bench aes -- --exact --nocapture

#![feature(test)]
extern crate test;
use test::Bencher;

use cck_symmetric::*;

#[bench]
fn aes_128_gcm_encrypt_in_place(b: &mut Bencher) {

    b.iter(|| {
        let mut message: cck_symmetric::ArrayVec<u8, 256> = cck_symmetric::ArrayVec::new();

        // hello: [104, 101, 108, 108, 111]
        message
            .extend_from_slice(&[104, 101, 108, 108, 111])
            .unwrap();
    
        cck_symmetric::aead_encrypt_in_place(
            &cck_symmetric::aes::aes_128_gcm(&[0u8; 16]),
            &[0u8; 12],
            &[],
            &mut message,
        )
        .unwrap()
    });
}

#[bench]
fn aes_128_gcm_decrypt_in_place(b: &mut Bencher) {
   

    b.iter(|| {
        let mut message: cck_symmetric::ArrayVec<u8, 256> = cck_symmetric::ArrayVec::new();

        message
            .extend_from_slice(&[
                107, 237, 182, 162, 15, 252, 5, 243, 71, 93, 41, 3, 76, 170, 28, 200, 250, 54, 40, 195,
                159,
            ])
            .unwrap();

        cck_symmetric::aead_decrypt_in_place(
            &cck_symmetric::aes::aes_128_gcm(&[0u8; 16]),
            &[0u8; 12],
            &[],
            &mut message,
        )
        .unwrap()
    });
}

#[cfg(feature = "alloc")]
#[bench]
fn aes_128_gcm_encrypt(b: &mut Bencher) {
    b.iter(|| {
        // hello: [104, 101, 108, 108, 111]
        cck_symmetric::aead_encrypt(
            &cck_symmetric::aes::aes_128_gcm(&[0u8; 16]),
            &[0u8; 12],
            &[],
            &[104, 101, 108, 108, 111],
        )
        .unwrap();
    })
}

#[cfg(feature = "alloc")]
#[bench]
fn aes_128_gcm_decrypt(b: &mut Bencher) {
    b.iter(|| {
        cck_symmetric::aead_decrypt(
            &cck_symmetric::aes::aes_128_gcm(&[0u8; 16]),
            &[0u8; 12],
            &[],
            &[
                107, 237, 182, 162, 15, 252, 5, 243, 71, 93, 41, 3, 76, 170, 28, 200, 250, 54, 40,
                195, 159,
            ],
        )
        .unwrap();
    })
}

#[bench]
fn aes_192_gcm_encrypt_in_place(b: &mut Bencher) {
 
    b.iter(|| {
        let mut message: cck_symmetric::ArrayVec<u8, 256> = cck_symmetric::ArrayVec::new();

        // hello: [104, 101, 108, 108, 111]
        message
            .extend_from_slice(&[104, 101, 108, 108, 111])
            .unwrap();

            
        cck_symmetric::aead_encrypt_in_place(
            &cck_symmetric::aes::aes_192_gcm(&[0u8; 24]),
            &[0u8; 12],
            &[],
            &mut message,
        )
        .unwrap()
    });
}

#[bench]
fn aes_192_gcm_decrypt_in_place(b: &mut Bencher) {


    b.iter(|| {
        let mut message: cck_symmetric::ArrayVec<u8, 256> = cck_symmetric::ArrayVec::new();

        message
            .extend_from_slice(&[
                240, 130, 72, 16, 104, 146, 96, 215, 100, 165, 207, 56, 143, 93, 38, 58, 208, 84, 62,
                136, 9,
            ])
            .unwrap();

        cck_symmetric::aead_decrypt_in_place(
            &cck_symmetric::aes::aes_192_gcm(&[0u8; 24]),
            &[0u8; 12],
            &[],
            &mut message,
        )
        .unwrap()
    });
}

#[cfg(feature = "alloc")]
#[bench]
fn aes_192_gcm_encrypt(b: &mut Bencher) {
    b.iter(|| {
        // hello: [104, 101, 108, 108, 111]
        cck_symmetric::aead_encrypt(
            &cck_symmetric::aes::aes_192_gcm(&[0u8; 24]),
            &[0u8; 12],
            &[],
            &[104, 101, 108, 108, 111],
        )
        .unwrap();
    })
}

#[cfg(feature = "alloc")]
#[bench]
fn aes_192_gcm_decrypt(b: &mut Bencher) {
    b.iter(|| {
        cck_symmetric::aead_decrypt(
            &cck_symmetric::aes::aes_192_gcm(&[0u8; 24]),
            &[0u8; 12],
            &[],
            &[
                240, 130, 72, 16, 104, 146, 96, 215, 100, 165, 207, 56, 143, 93, 38, 58, 208, 84,
                62, 136, 9,
            ],
        )
        .unwrap();
    })
}

#[bench]
fn aes_256_gcm_encrypt_in_place(b: &mut Bencher) {

    b.iter(|| {
        let mut message: cck_symmetric::ArrayVec<u8, 256> = cck_symmetric::ArrayVec::new();

        // hello: [104, 101, 108, 108, 111]
        message
            .extend_from_slice(&[104, 101, 108, 108, 111])
            .unwrap();

        cck_symmetric::aead_encrypt_in_place(
            &cck_symmetric::aes::aes_256_gcm(&[0u8; 32]),
            &[0u8; 12],
            &[],
            &mut message,
        )
        .unwrap()
    });
}

#[bench]
fn aes_256_gcm_decrypt_in_place(b: &mut Bencher) {


    b.iter(|| {
        let mut message: cck_symmetric::ArrayVec<u8, 256> = cck_symmetric::ArrayVec::new();

        message
            .extend_from_slice(&[
                166, 194, 44, 81, 34, 139, 144, 143, 127, 98, 255, 206, 166, 169, 47, 171, 239, 57,
                191, 77, 147,
            ])
            .unwrap();   

        cck_symmetric::aead_decrypt_in_place(
            &cck_symmetric::aes::aes_256_gcm(&[0u8; 32]),
            &[0u8; 12],
            &[],
            &mut message,
        )
        .unwrap()
    });
}

#[cfg(feature = "alloc")]
#[bench]
fn aes_256_gcm_encrypt(b: &mut Bencher) {
    b.iter(|| {
        // hello: [104, 101, 108, 108, 111]
        cck_symmetric::aead_encrypt(
            &cck_symmetric::aes::aes_256_gcm(&[0u8; 32]),
            &[0u8; 12],
            &[],
            &[104, 101, 108, 108, 111],
        )
        .unwrap();
    })
}

#[cfg(feature = "alloc")]
#[bench]
fn aes_256_gcm_decrypt(b: &mut Bencher) {
    b.iter(|| {
        cck_symmetric::aead_decrypt(
            &cck_symmetric::aes::aes_256_gcm(&[0u8; 32]),
            &[0u8; 12],
            &[],
            &[
                166, 194, 44, 81, 34, 139, 144, 143, 127, 98, 255, 206, 166, 169, 47, 171, 239, 57,
                191, 77, 147,
            ],
        )
        .unwrap();
    })
}
