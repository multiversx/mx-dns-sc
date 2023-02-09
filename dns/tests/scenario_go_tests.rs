#[test]
#[ignore = "there are differences in the emitted logs (transferValueOnly)"]
fn test_mandos_main_go() {
    multiversx_sc_scenario::run_go("scenarios/main.scen.json");
}

#[test]
fn test_mandos_resolve_elrond_go() {
    multiversx_sc_scenario::run_go("scenarios/resolve-elrond.scen.json");
}

#[test]
#[ignore = "migrateUserName builtin function not implemented yet"]
fn test_mandos_migrate_go() {
    multiversx_sc_scenario::run_go("scenarios/migrate.scen.json");
}
