#![no_std]
#![allow(unused_attributes)]
#![allow(clippy::string_lit_as_bytes)]

imports!();

#[elrond_wasm_derive::contract(UserMockImpl)]
pub trait UserMock {
    #[init]
    fn init(&self) {}

    #[storage_get("name")]
    fn get_name(&self) -> Vec<u8>;

    #[storage_set("name")]
    fn set_name(&self, name: &[u8]);

    #[endpoint(SetUserName)]
    fn set_user_name_endpoint(&self, name: &[u8]) -> SCResult<()> {
        let old_name = self.get_name();
        if !old_name.is_empty() {
            sc_error!("user name already set")
        } else {
            self.set_name(name);
            Ok(())
        }
    }
}
