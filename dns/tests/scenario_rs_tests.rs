use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("dns");

    blockchain.register_contract("file:output/dns.wasm", dns::ContractBuilder);
    blockchain
}

#[test]
fn test_mandos_main_rs() {
    multiversx_sc_scenario::run_rs("mandos/main.scen.json", world());
}

#[test]
fn test_mandos_resolve_elrond_rs() {
    multiversx_sc_scenario::run_rs("mandos/resolve-elrond.scen.json", world());
}

#[test]
#[ignore = "migrateUserName builtin function not implemented yet"]
fn test_mandos_migrate_rs() {
    multiversx_sc_scenario::run_rs("mandos/migrate.scen.json", world());
}
