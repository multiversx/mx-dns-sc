use multiversx_sc::types::ManagedBuffer;
use multiversx_sc_scenario::DebugApi;

fn validate_name(name_str: &str) -> Result<(), &'static str> {
    let mb = ManagedBuffer::<DebugApi>::from(name_str.as_bytes());
    dns::name_validation::validate_name(&mb)
}

fn check(name: &str, expected: &Result<(), &'static str>) {
    assert_eq!(validate_name(name), *expected);
}

#[test]
fn test_validate_name() {
    let _ = DebugApi::dummy();

    let ok = &Ok(());
    let wrong_suffix = &Err("wrong suffix");
    let name_is_too_short = &Err("name is too short");
    let character_not_allowed = &Err("character not allowed");
    let name_too_long = &Err("name too long");

    // .x
    check("aaa.x", ok);
    check("aaaaaaaaaa.x", ok);
    check("zzzzzzzzzz.x", ok);
    check("0000000000.x", ok);
    check("9999999999.x", ok);
    check("coolname0001.x", ok);

    // .elrond
    check("aaa.elrond", wrong_suffix);
    check("aaaaaaaaaa.elrond", wrong_suffix);
    check("zzzzzzzzzz.elrond", wrong_suffix);
    check("0000000000.elrond", wrong_suffix);
    check("9999999999.elrond", wrong_suffix);
    check("coolname0001.elrond", wrong_suffix);

    // too short
    check(".x", name_is_too_short);
    check("aa.x", name_is_too_short);
    check(".elrond", wrong_suffix);
    check("aa.elrond", wrong_suffix);

    // lowercase only
    check("Aaaaaaaaaa.x", character_not_allowed);
    check("Aaaaaaaaaa.elrond", wrong_suffix);

    // no other chars
    check("aaaaa.aaaa.x", character_not_allowed);
    check("aaaaa@aaaa.x", character_not_allowed);
    check("aaaaa+aaaa.x", character_not_allowed);
    check("aaaaa-aaaa.x", character_not_allowed);
    check("aaaaa_aaaa.x", character_not_allowed);
    check("aaaaa.aaaa.elrond", wrong_suffix);
    check("aaaaa@aaaa.elrond", wrong_suffix);
    check("aaaaa+aaaa.elrond", wrong_suffix);
    check("aaaaa-aaaa.elrond", wrong_suffix);
    check("aaaaa_aaaa.elrond", wrong_suffix);

    // without suffix
    check("aaaaaaaaaa", wrong_suffix);
    check("zzzzzzzzzz", wrong_suffix);
    check("0000000000", wrong_suffix);
    check("9999999999", wrong_suffix);
    check("coolname0001", wrong_suffix);

    // name too long
    check("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.x", ok); // 32 bytes
    check("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.x", name_too_long); // 33 bytes
    check("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.x", name_too_long); // 37 bytes
    check("aaaaaaaaaaaaaaaaaaaaaaaaa.elrond", wrong_suffix); // 32 bytes
    check("aaaaaaaaaaaaaaaaaaaaaaaaaa.elrond", name_too_long); // 33 bytes
    check("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.elrond", name_too_long); // 37 bytes
}
