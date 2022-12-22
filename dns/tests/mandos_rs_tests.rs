use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("dns");

    blockchain.register_contract(
        "file:output/elrond-wasm-sc-dns.wasm",
        elrond_wasm_sc_dns::ContractBuilder,
    );
    blockchain
}

#[test]
fn test_mandos_main_rs() {
    elrond_wasm_debug::mandos_rs("mandos/main.scen.json", world());
}

#[test]
fn test_mandos_resolve_elrond_rs() {
    elrond_wasm_debug::mandos_rs("mandos/resolve-elrond.scen.json", world());
}

#[test]
#[ignore = "migrateUserName builtin function not implemented yet"]
fn test_mandos_migrate_rs() {
    elrond_wasm_debug::mandos_rs("mandos/migrate.scen.json", world());
}
