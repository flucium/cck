// e.g.
// cargo test --package cck-hash --test blake3 --  --nocapture
// cargo test --package cck-hash --test blake3 -- blake3_digest --nocapture

#[test]
fn blake3_digest() {
    let hash = cck_hash::blake3::digest(&[], &[]);

    assert_eq!(hash.len(), 32);

    assert_eq!(
        hash,
        [
            175, 19, 73, 185, 245, 249, 161, 166, 160, 64, 77, 234, 54, 220, 201, 73, 155, 203, 37,
            201, 173, 193, 18, 183, 204, 154, 147, 202, 228, 31, 50, 98
        ]
    );
}

#[test]
fn blake3_derive_key() {
    let hash = cck_hash::blake3::derive_key("", &[], &[]);

    assert_eq!(hash.len(), 32);

    assert_eq!(
        hash,
        [
            116, 16, 17, 152, 149, 17, 224, 214, 181, 37, 50, 50, 13, 158, 219, 108, 13, 239, 10,
            183, 232, 50, 185, 155, 204, 18, 89, 89, 28, 226, 215, 91
        ]
    );
}

#[test]
fn blake3_mac() {
    let hash = cck_hash::blake3::mac(&[0u8; 32], &[], &[]);

    assert_eq!(hash.len(), 32);

    assert_eq!(
        hash,
        [
            167, 249, 28, 237, 5, 51, 193, 44, 213, 151, 6, 242, 220, 56, 194, 168, 195, 156, 0,
            122, 232, 154, 182, 73, 38, 152, 119, 140, 134, 132, 196, 131
        ]
    );
}
