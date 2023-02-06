use multiversx_sc::types::ManagedBuffer;
use multiversx_sc_scenario::DebugApi;
use multiversx_sc_sc_dns::name_validation::SuffixType;

fn prepare_name_for_hash_and_classify(name_str: &str) -> (ManagedBuffer<DebugApi>, SuffixType) {
    let mb = ManagedBuffer::<DebugApi>::from(name_str.as_bytes());
    multiversx_sc_sc_dns::name_validation::prepare_name_for_hash_and_classify(&mb)
}

fn check(name: &str, expected: &str, suffix_type: SuffixType) {
    assert_eq!(
        prepare_name_for_hash_and_classify(name),
        (ManagedBuffer::from(expected.as_bytes()), suffix_type)
    );
}

#[test]
fn prepare_name_for_hash_test() {
    let _ = DebugApi::dummy();

    // .x is replaced with .elrond
    check("aaa.x", "aaa.elrond", SuffixType::X);
    check("aaaaaaaaaa.x", "aaaaaaaaaa.elrond", SuffixType::X);
    check("zzzzzzzzzz.x", "zzzzzzzzzz.elrond", SuffixType::X);
    check("0000000000.x", "0000000000.elrond", SuffixType::X);
    check("9999999999.x", "9999999999.elrond", SuffixType::X);
    check("coolname0001.x", "coolname0001.elrond", SuffixType::X);

    // .elrond names are returned unchanged
    check("aaa.elrond", "aaa.elrond", SuffixType::Elrond);
    check("aaaaaaaaaa.elrond", "aaaaaaaaaa.elrond", SuffixType::Elrond);
    check("zzzzzzzzzz.elrond", "zzzzzzzzzz.elrond", SuffixType::Elrond);
    check("0000000000.elrond", "0000000000.elrond", SuffixType::Elrond);
    check("9999999999.elrond", "9999999999.elrond", SuffixType::Elrond);
    check(
        "coolname0001.elrond",
        "coolname0001.elrond",
        SuffixType::Elrond,
    );

    // undefined behavior for invalid names
    // tests are only for ensuring that the function doesn't panic
    check("aa.x", "aa.elrond", SuffixType::X);
    check("test.abc.x", "test.abc.elrond", SuffixType::X);
    check("test.abc.foo", "test.abc.foo", SuffixType::Elrond);
    check("test", "test", SuffixType::Elrond);
    check("test.abc", "test.abc", SuffixType::Elrond);
}
