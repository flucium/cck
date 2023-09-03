// e.g.
// cargo bench --package cck --bench compress -- --exact --nocapture
// cargo bench --package cck --bench compress -- deflate_compress --exact --nocapture
// cargo bench --features=alloc --package cck --bench compress -- deflate_compress_vec --exact --nocapture

#![feature(test)]
extern crate test;
use test::Bencher;

#[bench]
fn deflate_compress(b: &mut Bencher) {
    let bytes = b"hello";
    let mut buffer: [u8; 10] = [0u8; 10];

    b.iter(|| {
        cck::compress::deflate::compress(cck::compress::Level::default(), bytes, &mut buffer)
            .unwrap();
    });
}

#[bench]
fn deflate_decompress(b: &mut Bencher) {
    let bytes: [u8; 7] = [203, 72, 205, 201, 201, 7, 0];
    let mut buffer: [u8; 10] = [0u8; 10];
    b.iter(|| {
        cck::compress::deflate::decompress(cck::compress::Level::default(), bytes, &mut buffer)
            .unwrap();
    });
}

#[cfg(feature = "alloc")]
#[bench]
fn deflate_compress_vec(b: &mut Bencher) {
    let bytes = b"hello";

    b.iter(|| {
        cck::compress::deflate::compress_vec(cck::compress::Level::default(), bytes).unwrap();
    });
}

#[cfg(feature = "alloc")]
#[bench]
fn deflate_decompress_vec(b: &mut Bencher) {
    let bytes: [u8; 7] = [203, 72, 205, 201, 201, 7, 0];

    b.iter(|| {
        cck::compress::deflate::decompress_vec(cck::compress::Level::default(), bytes).unwrap();
    });
}
