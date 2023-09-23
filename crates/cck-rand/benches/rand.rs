// e.g.
// cargo bench --package cck-rand --bench rand -- --exact --nocapture
// cargo bench --package cck-rand --bench rand -- gen_16 --exact --nocapture
// cargo bench --package cck-rand --bench rand -- gen_24 --exact --nocapture
// cargo bench --package cck-rand --bench rand -- gen_32 --exact --nocapture
// cargo bench --package cck-rand --bench rand -- gen_16_ascii --exact --nocapture
// cargo bench --package cck-rand --bench rand -- gen_24_ascii --exact --nocapture
// cargo bench --package cck-rand --bench rand -- gen_32_ascii --exact --nocapture

#![feature(test)]
extern crate test;
use test::Bencher;

#[bench]
fn gen_16(b: &mut Bencher) {
    b.iter(|| {
        cck_rand::gen_16();
    });
}

#[bench]
fn gen_24(b: &mut Bencher) {
    b.iter(|| {
        cck_rand::gen_24();
    });
}

#[bench]
fn gen_32(b: &mut Bencher) {
    b.iter(|| {
        cck_rand::gen_32();
    });
}

#[bench]
fn gen_16_ascii(b: &mut Bencher) {
    b.iter(|| {
        cck_rand::gen_16_ascii();
    });
}

#[bench]
fn gen_24_ascii(b: &mut Bencher) {
    b.iter(|| {
        cck_rand::gen_24_ascii();
    });
}

#[bench]
fn gen_32_ascii(b: &mut Bencher) {
    b.iter(|| {
        cck_rand::gen_32_ascii();
    });
}
