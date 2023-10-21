// e.g.
// cargo test --package cck-asymmetric --test x25519 --  --nocapture
// cargo test --package cck-asymmetric --test x25519 -- x25519_diffie_hellman_self --nocapture

#[test]
fn x25519_diffie_hellman_self() {
    const PRIVATE_KEY: [u8; 32] = [
        47, 32, 20, 152, 110, 123, 218, 98, 27, 10, 200, 205, 134, 208, 230, 55, 210, 149, 173, 48,
        64, 181, 205, 229, 42, 39, 117, 120, 106, 53, 244, 25,
    ];

    const PUBLIC_KEY: [u8; 32] = [
        228, 182, 160, 196, 88, 133, 155, 51, 7, 208, 34, 252, 148, 139, 105, 71, 223, 35, 54, 190,
        144, 34, 110, 34, 236, 39, 163, 14, 204, 8, 2, 59,
    ];

    const SHARED_SECRET: [u8; 32] = [
        77, 221, 140, 129, 97, 175, 165, 89, 63, 49, 27, 12, 183, 119, 73, 24, 243, 254, 39, 106,
        131, 13, 211, 10, 159, 14, 189, 57, 197, 238, 122, 44,
    ];

    assert_eq!(
        cck_asymmetric::x25519::diffie_hellman(&PRIVATE_KEY, &PUBLIC_KEY),
        SHARED_SECRET
    );
}

#[test]
fn x25519_diffie_hellman_their() {
    let alice_private_key = cck_asymmetric::x25519::gen_private_key();

    let alice_public_key = cck_asymmetric::x25519::gen_public_key(&alice_private_key);

    let bob_private_key = cck_asymmetric::x25519::gen_private_key();

    let bob_public_key = cck_asymmetric::x25519::gen_public_key(&bob_private_key);

    let alice_shared_secret =
        cck_asymmetric::x25519::diffie_hellman(&alice_private_key, &bob_public_key);

    let bob_shared_secret =
        cck_asymmetric::x25519::diffie_hellman(&bob_private_key, &alice_public_key);

    assert_eq!(alice_shared_secret, bob_shared_secret);
}
