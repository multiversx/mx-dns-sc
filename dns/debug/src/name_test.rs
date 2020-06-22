use dns::name_validation::*;

#[test]
fn test_validate_name() {
    assert!(validate_name(&*b"aaa").is_ok());
    assert!(validate_name(&*b"zzz").is_ok());
    assert!(validate_name(&*b"AAA").is_ok());
    assert!(validate_name(&*b"ZZZ").is_ok());
    assert!(validate_name(&*b"000").is_ok());
    assert!(validate_name(&*b"999").is_ok());
    assert!(validate_name(&*b"coolname0001").is_ok());
}