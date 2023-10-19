// e.g.
// cargo test --package cck-hash --test sha2 --  --nocapture
// cargo test --package cck-hash --test sha2 -- sha256_digest --nocapture

#[test]
fn sha256_digest() {
    let hash = cck_hash::sha2::sha256_digest(&[], &[]);

    assert_eq!(hash.len(), 32);

    assert_eq!(
        hash,
        [
            227, 176, 196, 66, 152, 252, 28, 20, 154, 251, 244, 200, 153, 111, 185, 36, 39, 174,
            65, 228, 100, 155, 147, 76, 164, 149, 153, 27, 120, 82, 184, 85
        ]
    );
}

#[test]
fn sha512_digest() {
    let hash = cck_hash::sha2::sha512_digest(&[], &[]);

    assert_eq!(hash.len(), 64);

    assert_eq!(
        hash,
        [
            207, 131, 225, 53, 126, 239, 184, 189, 241, 84, 40, 80, 214, 109, 128, 7, 214, 32, 228,
            5, 11, 87, 21, 220, 131, 244, 169, 33, 211, 108, 233, 206, 71, 208, 209, 60, 93, 133,
            242, 176, 255, 131, 24, 210, 135, 126, 236, 47, 99, 185, 49, 189, 71, 65, 122, 129,
            165, 56, 50, 122, 249, 39, 218, 62
        ]
    );
}

#[test]
fn sha512_256_digest() {
    let hash = cck_hash::sha2::sha512_256_digest(&[], &[]);

    assert_eq!(hash.len(), 32);

    assert_eq!(
        hash,
        [
            198, 114, 184, 209, 239, 86, 237, 40, 171, 135, 195, 98, 44, 81, 20, 6, 155, 221, 58,
            215, 184, 249, 115, 116, 152, 208, 192, 30, 206, 240, 150, 122
        ]
    );
}
