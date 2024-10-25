#[cfg(feature = "serde")]
use a1::new;

#[test]
#[cfg(feature = "serde")]
fn test_a1_to_and_from() {
    assert_eq!("A1", new("A1").unwrap().to_string());
}
