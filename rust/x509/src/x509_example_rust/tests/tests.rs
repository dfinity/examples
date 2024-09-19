use std::convert::TryFrom;
use x509_example_rust::SchnorrKeyName;

#[test]
fn test_expected_strum() {
    assert_eq!(
        <&'static str>::try_from(SchnorrKeyName::DfxTestKey),
        Ok("dfx_test_key")
    );
    assert_eq!(
        <&'static str>::try_from(SchnorrKeyName::TestKey1),
        Ok("test_key_1")
    );
    assert_eq!(<&'static str>::try_from(SchnorrKeyName::Key1), Ok("key_1"));
}
