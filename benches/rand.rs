// e.g.
// cargo bench --package cck --bench rand -- --exact --nocapture
// cargo bench --package cck --bench rand -- gen_32 --exact --nocapture
// cargo bench --package cck --bench rand -- gen_12 --exact --nocapture 

#![feature(test)]
extern crate test;
use test::Bencher;

#[bench]
fn gen_32(b: &mut Bencher) {
    b.iter(|| cck::rand::gen_32());
}

#[bench]
fn gen_32_ascii(b: &mut Bencher) {
    b.iter(|| cck::rand::gen_32_ascii());
}

#[bench]
fn gen_24(b: &mut Bencher) {
    b.iter(|| cck::rand::gen_24());
}

#[bench]
fn gen_24_ascii(b: &mut Bencher) {
    b.iter(|| cck::rand::gen_24_ascii());
}

#[bench]
fn gen_12(b: &mut Bencher) {
    b.iter(|| cck::rand::gen_12());
}

#[bench]
fn gen_12_ascii(b: &mut Bencher) {
    b.iter(|| cck::rand::gen_12_ascii());
}