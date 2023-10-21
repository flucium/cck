// e.g.
// cargo bench --package cck-asymmetric --bench x25519 -- --exact --nocapture
// cargo bench --package cck-asymmetric --bench x25519 -- x25519_gen_private_key --exact --nocapture

#![feature(test)]
extern crate test;
use test::Bencher;


#[bench]
fn x25519_gen_private_key(b: &mut Bencher) {
    b.iter(|| cck_asymmetric::x25519::gen_private_key());
}

#[bench]
fn x25519_gen_public_key(b: &mut Bencher) {
    let private_key = cck_asymmetric::x25519::gen_private_key();
    b.iter(|| cck_asymmetric::x25519::gen_public_key(&private_key));
}

#[bench]
fn x25519_diffie_hellman(b: &mut Bencher) {
    const PRIVATE_KEY: [u8; 32] = [
        45, 162, 45, 39, 64, 231, 153, 194, 122, 98, 107, 62, 92, 11, 143, 141, 125, 225, 86, 3,
        112, 134, 89, 217, 7, 69, 94, 221, 58, 144, 165, 180,
    ];

    const PUBLIC_KEY: [u8; 32] = [
        199, 120, 240, 236, 92, 200, 1, 149, 127, 9, 188, 222, 135, 251, 137, 2, 128, 66, 72, 94,
        134, 137, 212, 88, 80, 229, 179, 223, 163, 149, 187, 10,
    ];

    b.iter(|| cck_asymmetric::x25519::diffie_hellman(&PRIVATE_KEY, &PUBLIC_KEY));
}
