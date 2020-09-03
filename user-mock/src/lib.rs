
#![no_std]
#![no_main]
#![allow(unused_attributes)]

imports!();

#[elrond_wasm_derive::contract(UserMockImpl)]
pub trait UserMock {

    #[init]
    fn init(&self) {
    }

    #[storage_get("name_hash")]
    fn get_name_hash(&self) -> Vec<u8>;

    #[storage_set("name_hash")]
    fn set_name_hash(&self, name_hash: &[u8]);

    #[endpoint(SetUserName)]
    fn set_user_name_endpoint(&self, name: Vec<u8>) -> SCResult<()> {
        let old_name_hash = self.get_name_hash();
        if old_name_hash.len() > 0 {
            sc_error!("user name already set")
        } else {
            self.set_name_hash(name.as_slice());
            Ok(())
        }
    }

}
