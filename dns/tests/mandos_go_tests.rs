#[test]
#[ignore = "there are differences in the emitted logs (transferValueOnly)"]
fn test_mandos_main_go() {
    elrond_wasm_debug::mandos_go("mandos/main.scen.json");
}

#[test]
fn test_mandos_resolve_elrond_go() {
    elrond_wasm_debug::mandos_go("mandos/resolve-elrond.scen.json");
}

#[test]
#[ignore = "migrateUserName builtin function not implemented yet"]
fn test_mandos_migrate_go() {
    elrond_wasm_debug::mandos_go("mandos/migrate.scen.json");
}
