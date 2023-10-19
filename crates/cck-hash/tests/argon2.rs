// e.g.
// cargo test --package cck-hash --test argon2 --  --nocapture

#[test]
fn argon2i() {
    assert_eq!(
        cck_hash::argon2::digest(&cck_hash::argon2::argon2i(), &[0u8; 32], &[0u8; 32]).unwrap(),
        [
            135, 139, 84, 69, 8, 95, 232, 138, 80, 200, 153, 104, 144, 2, 22, 234, 134, 180, 155,
            11, 250, 173, 50, 219, 184, 88, 73, 46, 48, 239, 143, 112
        ]
    );
}

#[test]
fn argon2d() {
    assert_eq!(
        cck_hash::argon2::digest(&cck_hash::argon2::argon2d(), &[0u8; 32], &[0u8; 32]).unwrap(),
        [
            194, 220, 4, 97, 76, 131, 176, 32, 189, 0, 29, 211, 61, 22, 0, 105, 90, 54, 187, 63,
            94, 210, 237, 36, 151, 112, 251, 163, 196, 244, 241, 172
        ]
    );
}

#[test]
fn argon2id() {
    assert_eq!(
        cck_hash::argon2::digest(&cck_hash::argon2::argon2id(), &[0u8; 32], &[0u8; 32]).unwrap(),
        [
            198, 56, 152, 213, 235, 147, 21, 57, 221, 90, 85, 125, 247, 228, 77, 125, 185, 146,
            213, 64, 239, 254, 221, 196, 20, 163, 105, 76, 94, 4, 136, 201
        ]
    );
}

#[test]
fn argon2id_err() {
    // error: salt size
    assert_eq!(
        cck_hash::argon2::digest(&cck_hash::argon2::argon2id(), &[], &[]).is_err(),
        true
    );

    // error: salt size
    assert_eq!(
        cck_hash::argon2::digest(&cck_hash::argon2::argon2id(), &[], &[0u8; 7]).is_err(),
        true
    );

    // ok: salt size 8 ~
    assert_eq!(
        cck_hash::argon2::digest(&cck_hash::argon2::argon2id(), &[], &[0u8; 8]).is_ok(),
        true
    );
}
