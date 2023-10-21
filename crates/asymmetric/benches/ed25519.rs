// e.g.
// cargo bench --package cck-asymmetric --bench ed25519 -- --exact --nocapture
// cargo bench --package cck-asymmetric --bench ed25519 -- ed25519_sign --exact --nocapture

#![feature(test)]
extern crate test;
use test::Bencher;

// #[bench]
// fn ed25519_gen_keypair(b: &mut Bencher) {
//     b.iter(|| cck_asymmetric::ed25519_gen_keypair());
// }

#[bench]
fn ed25519_gen_private_key(b: &mut Bencher) {
    b.iter(|| cck_asymmetric::ed25519::gen_private_key());
}

#[bench]
fn ed25519_gen_public_key(b: &mut Bencher) {
    let private_key = cck_asymmetric::ed25519::gen_private_key();
    b.iter(|| cck_asymmetric::ed25519::gen_public_key(&private_key));
}

#[bench]
fn ed25519_sign(b: &mut Bencher) {
    const PRIVATE_KEY: [u8; 32] = [
        68, 87, 109, 156, 131, 213, 127, 10, 63, 10, 61, 181, 243, 100, 121, 102, 53, 62, 215, 212,
        67, 223, 238, 9, 34, 39, 44, 10, 51, 2, 56, 96,
    ];

    // hello: [104, 101, 108, 108, 111]
    const MESSAGE: [u8; 5] = [104, 101, 108, 108, 111];

    b.iter(|| cck_asymmetric::ed25519::sign(&PRIVATE_KEY, &MESSAGE));
}

#[bench]
fn ed25519_verify(b: &mut Bencher) {
    const PUBLIC_KEY: [u8; 32] = [
        8, 230, 98, 51, 57, 27, 17, 99, 190, 212, 187, 167, 138, 235, 172, 89, 144, 104, 152, 174,
        242, 25, 168, 132, 53, 182, 187, 232, 142, 1, 1, 187,
    ];

    // hello: [104, 101, 108, 108, 111]
    const MESSAGE: [u8; 5] = [104, 101, 108, 108, 111];

    const SIGNATURE: [u8; 64] = [
        83, 20, 131, 218, 63, 174, 163, 255, 37, 122, 54, 8, 232, 117, 239, 45, 201, 70, 101, 142,
        217, 147, 210, 94, 135, 222, 113, 244, 162, 251, 115, 56, 222, 63, 84, 150, 241, 44, 243,
        138, 57, 64, 22, 0, 105, 198, 207, 240, 52, 170, 213, 157, 88, 49, 176, 187, 42, 12, 53,
        79, 41, 22, 42, 3,
    ];

    b.iter(|| cck_asymmetric::ed25519::verify(&PUBLIC_KEY, &MESSAGE, &SIGNATURE));
}
