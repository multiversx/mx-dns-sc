use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::DebugApi;

fn validate_name(name_str: &str) -> Result<(), &'static str> {
    let mb = ManagedBuffer::<DebugApi>::from(name_str.as_bytes());
    elrond_wasm_sc_dns::name_validation::validate_name(&mb)
}

#[test]
fn test_validate_name() {
    let _ = DebugApi::dummy();

    // ok
    assert!(validate_name("aaaaaaaaaa.elrond").is_ok());
    assert!(validate_name("zzzzzzzzzz.elrond").is_ok());
    assert!(validate_name("0000000000.elrond").is_ok());
    assert!(validate_name("9999999999.elrond").is_ok());
    assert!(validate_name("coolname0001.elrond").is_ok());

    // too short
    assert!(validate_name(".elrond").is_err());
    assert!(validate_name("aa.elrond").is_err());

    // lowercase only
    assert!(validate_name("Aaaaaaaaaa.elrond").is_err());

    // no other chars
    assert!(validate_name("aaaaa.aaaa.elrond").is_err());
    assert!(validate_name("aaaaa@aaaa.elrond").is_err());
    assert!(validate_name("aaaaa+aaaa.elrond").is_err());
    assert!(validate_name("aaaaa-aaaa.elrond").is_err());
    assert!(validate_name("aaaaa_aaaa.elrond").is_err());

    // without suffix
    assert!(validate_name("aaaaaaaaaa").is_err());
    assert!(validate_name("zzzzzzzzzz").is_err());
    assert!(validate_name("0000000000").is_err());
    assert!(validate_name("9999999999").is_err());
    assert!(validate_name("coolname0001").is_err());

    // name too long
    assert!(validate_name("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.elrond").is_err());
}
