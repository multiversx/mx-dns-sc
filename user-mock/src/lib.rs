
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
    fn _get_name_hash(&self) -> Vec<u8>;

    #[storage_set("name_hash")]
    fn _set_name_hash(&self, name_hash: &[u8]);

    #[endpoint(SetUserName)]
    fn set_user_name_endpoint(&self, name_hash: H256) -> SCResult<()> {
        let old_name_hash = self._get_name_hash();
        if old_name_hash.len() > 0 {
            sc_error!("user name already set")
        } else {
            self._set_name_hash(name_hash.as_bytes());
            Ok(())
        }
    }

}
