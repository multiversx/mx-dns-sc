
#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(unused_attributes)]

imports!();

#[elrond_wasm_derive::contract(UserMockImpl)]
pub trait UserMock {

    fn init(&self) {
    }

    #[private]
    #[storage_get("name_hash")]
    fn _get_name_hash(&self) -> Vec<u8>;

    #[private]
    #[storage_set("name_hash")]
    fn _set_name_hash(&self, name_hash: &[u8]);

    fn SetUserName(&self, name_hash: H256) -> Result<(), SCError> {
        let old_name_hash = self._get_name_hash();
        if old_name_hash.len() > 0 {
            sc_error!("user name already set")
        } else {
            self._set_name_hash(name_hash.as_bytes());
            Ok(())
        }
    }

}
