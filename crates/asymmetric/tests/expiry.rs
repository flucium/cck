
#[test]
fn expiry_default() {
    
    let expiry = keyring::Expiry::default();

    assert_eq!(expiry.year(), (0, 0, 0, 0));

    assert_eq!(expiry.month(), (0, 0));

    assert_eq!(expiry.day(), (0, 0));
}

#[test]
fn expiry_from_pattern1() {
    
    let expiry = keyring::Expiry::from((2, 0, 2, 3, 1, 2, 3, 1));

    assert_eq!(expiry.year(), (2, 0, 2, 3));

    assert_eq!(expiry.month(), (1, 2));

    assert_eq!(expiry.day(), (3, 1));
}

#[test]
fn expiry_from_pattern2() {

    let expiry = keyring::Expiry::from((2, 0, 2, 3, 0, 2, 0, 1));

    assert_eq!(expiry.year(), (2, 0, 2, 3));

    assert_eq!(expiry.month(), (0, 2));

    assert_eq!(expiry.day(), (0, 1));
}
