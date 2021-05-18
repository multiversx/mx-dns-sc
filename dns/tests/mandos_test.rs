use elrond_wasm::*;
use elrond_wasm_debug::*;

#[allow(dead_code)]
fn contract_map() -> ContractMap<TxContext> {
    let mut contract_map = ContractMap::new();
    contract_map.register_contract(
        "file:../output/dns.wasm",
        Box::new(|context| Box::new(elrond_wasm_sc_dns::contract_obj(context))),
    );
    contract_map
}

#[test]
fn test_mandos_main() {
    parse_execute_mandos("mandos/main.scen.json", &contract_map());
}
