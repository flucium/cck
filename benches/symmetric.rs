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
fn aes_128_gcm_encrypt_in_place(b: &mut Bencher) {
    b.iter(|| {
        let mut message: cck::symmetric::ArrayVec<u8, 256> = cck::symmetric::ArrayVec::new();

        // hello: [104, 101, 108, 108, 111]
        message
            .extend_from_slice(&[104, 101, 108, 108, 111])
            .unwrap();

        cck::symmetric::aead_encrypt_in_place(
            &cck::symmetric::aes_128_gcm(&[0u8; 16]),
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
        let mut message: cck::symmetric::ArrayVec<u8, 256> = cck::symmetric::ArrayVec::new();

        message
            .extend_from_slice(&[
                107, 237, 182, 162, 15, 252, 5, 243, 71, 93, 41, 3, 76, 170, 28, 200, 250, 54, 40,
                195, 159,
            ])
            .unwrap();

        cck::symmetric::aead_decrypt_in_place(
            &cck::symmetric::aes_128_gcm(&[0u8; 16]),
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
        cck::symmetric::aead_encrypt(
            &cck::symmetric::aes_128_gcm(&[0u8; 16]),
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
        cck::symmetric::aead_decrypt(
            &cck::symmetric::aes_128_gcm(&[0u8; 16]),
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
        let mut message: cck::symmetric::ArrayVec<u8, 256> = cck::symmetric::ArrayVec::new();

        // hello: [104, 101, 108, 108, 111]
        message
            .extend_from_slice(&[104, 101, 108, 108, 111])
            .unwrap();

        cck::symmetric::aead_encrypt_in_place(
            &cck::symmetric::aes_192_gcm(&[0u8; 24]),
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
        let mut message: cck::symmetric::ArrayVec<u8, 256> = cck::symmetric::ArrayVec::new();

        message
            .extend_from_slice(&[
                240, 130, 72, 16, 104, 146, 96, 215, 100, 165, 207, 56, 143, 93, 38, 58, 208, 84,
                62, 136, 9,
            ])
            .unwrap();

        cck::symmetric::aead_decrypt_in_place(
            &cck::symmetric::aes_192_gcm(&[0u8; 24]),
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
        cck::symmetric::aead_encrypt(
            &cck::symmetric::aes_192_gcm(&[0u8; 24]),
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
        cck::symmetric::aead_decrypt(
            &cck::symmetric::aes_192_gcm(&[0u8; 24]),
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
        let mut message: cck::symmetric::ArrayVec<u8, 256> = cck::symmetric::ArrayVec::new();

        // hello: [104, 101, 108, 108, 111]
        message
            .extend_from_slice(&[104, 101, 108, 108, 111])
            .unwrap();

        cck::symmetric::aead_encrypt_in_place(
            &cck::symmetric::aes_256_gcm(&[0u8; 32]),
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
        let mut message: cck::symmetric::ArrayVec<u8, 256> = cck::symmetric::ArrayVec::new();

        message
            .extend_from_slice(&[
                166, 194, 44, 81, 34, 139, 144, 143, 127, 98, 255, 206, 166, 169, 47, 171, 239, 57,
                191, 77, 147,
            ])
            .unwrap();

        cck::symmetric::aead_decrypt_in_place(
            &cck::symmetric::aes_256_gcm(&[0u8; 32]),
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
        cck::symmetric::aead_encrypt(
            &cck::symmetric::aes_256_gcm(&[0u8; 32]),
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
        cck::symmetric::aead_decrypt(
            &cck::symmetric::aes_256_gcm(&[0u8; 32]),
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

#[bench]
fn xchacha20_poly1305_encrypt_in_place(b: &mut Bencher) {
    b.iter(|| {
        let mut message: cck::symmetric::ArrayVec<u8, 256> = cck::symmetric::ArrayVec::new();

        // hello: [104, 101, 108, 108, 111]
        message
            .extend_from_slice(&[104, 101, 108, 108, 111])
            .unwrap();

        cck::symmetric::aead_encrypt_in_place(
            &cck::symmetric::xchacha20poly1305(&[0u8; 32]),
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
        let mut message: cck::symmetric::ArrayVec<u8, 256> = cck::symmetric::ArrayVec::new();

        message
            .try_extend_from_slice(&[
                16, 251, 250, 229, 138, 249, 8, 112, 183, 173, 62, 24, 90, 50, 250, 154, 190, 134,
                87, 188, 17,
            ])
            .unwrap();

        cck::symmetric::aead_decrypt_in_place(
            &cck::symmetric::xchacha20poly1305(&[0u8; 32]),
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
        cck::symmetric::aead_encrypt(
            &cck::symmetric::xchacha20poly1305(&[0u8; 32]),
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
        cck::symmetric::aead_decrypt(
            &cck::symmetric::xchacha20poly1305(&[0u8; 32]),
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
