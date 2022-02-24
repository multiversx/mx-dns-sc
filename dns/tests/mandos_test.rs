use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("dns");

    blockchain
        .register_contract_builder("file:output/elrond-wasm-sc-dns.wasm", elrond_wasm_sc_dns::ContractBuilder);
    blockchain
}

#[test]
fn test_mandos_main_rs() {
    elrond_wasm_debug::mandos_rs("mandos/main.scen.json", world());
}
