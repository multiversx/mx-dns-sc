elrond_wasm::imports!();

#[elrond_wasm::derive::proxy]
pub trait UserBuiltin {
    #[endpoint(SetUserName)]
    fn set_user_name(&self, name: &ManagedBuffer) -> BigUint;
}
