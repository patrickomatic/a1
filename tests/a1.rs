//! Tests that parsing from and back to a string yields the same results (reflexive property)
use a1::A1;
use std::str::FromStr;

#[test]
fn test_a1_to_and_from() {
    assert_eq!("A1", A1::from_str("A1").unwrap().to_string());
    assert_eq!("Foo!A:C", A1::from_str("Foo!A:C").unwrap().to_string());
    assert_eq!(
        "'Foo Bar'!A:C",
        A1::from_str("'Foo Bar'!A:C").unwrap().to_string()
    );
}

#[test]
fn test_a1_to_and_from_absolute() {
    assert_eq!("$A$1", A1::from_str("$A$1").unwrap().to_string());

    assert_eq!("Foo!$A:$C", A1::from_str("Foo!$A:$C").unwrap().to_string());
}
