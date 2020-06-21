
#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(unused_attributes)]

imports!();

#[elrond_wasm_derive::contract(UserMockImpl)]
pub trait UserMock {

    fn init(&self) {
    }

    fn load(&self, key: &StorageKey) -> Vec<u8> {
        self.storage_load(key.as_bytes())
    }

    fn storeOverwrite(&self, key: &StorageKey, value: Vec<u8>) {
        self.storage_store(key.as_bytes(), value.as_slice());
    }

    fn storeIfNotExists(&self, key: StorageKey, value: Vec<u8>) -> bool {
        if self.storage_load_len(key.as_bytes()) > 0 {
            false
        } else {
            self.storage_store(key.as_bytes(), value.as_slice());
            true
        }
    }

}
