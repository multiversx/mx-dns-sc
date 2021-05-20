elrond_wasm::imports!();

#[elrond_wasm_derive::proxy]
pub trait UserBuiltin {
    #[endpoint(SetUserName)]
    fn set_user_name(&self, name: &BoxedBytes) -> Self::BigUint;
}
