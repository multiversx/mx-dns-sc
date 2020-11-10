extern crate dns;
use dns::*;
extern crate user_mock;
use elrond_wasm::*;
use elrond_wasm_debug::*;
use user_mock::*;

fn contract_map() -> ContractMap<TxContext> {
    let mut contract_map = ContractMap::new();
    contract_map.register_contract(
        "file:../output/dns.wasm",
        Box::new(|context| Box::new(DnsImpl::new(context))),
    );
    contract_map.register_contract(
        "file:../../user-mock/output/user-mock.wasm",
        Box::new(|context| Box::new(UserMockImpl::new(context))),
    );
    contract_map
}

#[test]
fn test_mandos_main() {
    parse_execute_mandos("mandos/main.scen.json", &contract_map());
}
