
#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(unused_attributes)]

pub mod name_util;
pub mod value_state;

use value_state::*;

imports!();

fn shard_id(addr: &Address, shard_mask: u8) -> u8 {
    let last_byte = addr.as_bytes()[31];
    last_byte & shard_mask
}

#[elrond_wasm_derive::callable(UserProxy)]
pub trait User {
    #[callback(userStoreCallback)]
    fn storeIfNotExists(&self,
        #[callback_arg] name_hash: &H256,
        key: &Vec<u8>,
        name: &Vec<u8>);
}

#[elrond_wasm_derive::contract(DnsImpl)]
pub trait Dns {

    fn init(&self,
            user_storage_key: Vec<u8>,
            shard_id_bits: usize) -> Result<(), SCError> {
        
        self._set_owner(&self.get_caller());
        self._set_user_storage_key(user_storage_key);

        if shard_id_bits > 8 {
            return sc_error!("shard_id_bits out of range");
        }
        self._set_shard_id_bits(shard_id_bits);

        Ok(())
    }

    fn register(&self, name: Vec<u8>, address: Address) -> Result<(), SCError>  {
        name_util::validate_name(&name.as_slice())?;

        let (name_hash, name_shard_id) = self.name_hash_shard(&name);
        if name_shard_id != self.getOwnShardId() {
            return sc_error!("name belongs to another shard");
        }

        let vs = self._get_value_state(&name_hash);
        if !vs.is_available() {
            return sc_error!("name already taken");
        }

        self._set_value_state(&name_hash, &ValueState::Pending(address.clone()));

        let user_proxy = contract_proxy!(self, &address, User);
        user_proxy.storeIfNotExists(
            &name_hash,
            &self.getUserStorageKey(),
            &name);

        return Ok(())
    }

    #[callback]
    fn userStoreCallback(&self, 
            result: AsyncCallResult<bool>,
            #[callback_arg] name_hash: H256) {

        match result {
            AsyncCallResult::Ok(success) => {
                if success {
                    // commit
                    let vm = self._get_value_state(&name_hash);
                    if let ValueState::Pending(addr) = vm {
                        self._set_value_state(&name_hash, &ValueState::Committed(addr));
                    }
                } else {
                    // revert
                    self._set_value_state(&name_hash, &ValueState::None);
                }
            },
            AsyncCallResult::Err(_) => {
                // also revert
                self._set_value_state(&name_hash, &ValueState::None);
            }
        }
        
    }

    fn resolve(&self, name: Vec<u8>) -> Option<Address> {
        let (name_hash, name_shard_id) = self.name_hash_shard(&name);
        if name_shard_id != self.getOwnShardId() {
            return None;
        }

        let vs = self._get_value_state(&name_hash);
        match vs {
            ValueState::Committed(address) => Some(address),
            _ => None
        }
    }

    #[private]
    fn name_hash_shard(&self, name: &Vec<u8>) -> (StorageKey, u8) {
        let name_hash = self.keccak256(&name);
        let shard_mask = self.getShardMask();
        let shard = name_hash[31] & shard_mask;
        (name_hash.into(), shard)
    }

    // STORAGE

    #[view]
    #[storage_get("owner")]
    fn getContractOwner(&self) -> Address;

    #[private]
    #[storage_set("owner")]
    fn _set_owner(&self, owner: &Address);

    #[view]
    #[storage_get("shard_id_bits")]
    fn getShardIdBits(&self) -> usize;

    #[private]
    #[storage_set("shard_id_bits")]
    fn _set_shard_id_bits(&self, shard_id_bits: usize);

    #[private]
    fn getShardMask(&self) -> u8 {
        (1 << self.getShardIdBits()) - 1
    }

    #[view]
    fn getOwnShardId(&self) -> u8 {
        shard_id(&self.get_own_address(), self.getShardMask())
    }

    #[view]
    #[storage_get("user_storage_key")]
    fn getUserStorageKey(&self) -> Vec<u8>;

    #[private]
    #[storage_set("user_storage_key")]
    fn _set_user_storage_key(&self, user_storage_key: Vec<u8>);

    #[private]
    #[storage_get("value_state")]
    fn _get_value_state(&self, name_hash: &H256) -> ValueState;

    #[private]
    #[storage_set("value_state")]
    fn _set_value_state(&self, name_hash: &H256, value_state: &ValueState);

    // UTILS

    #[view]
    fn nameShard(&self, name: Vec<u8>) -> u8 {
        self.name_hash_shard(&name).1
    }

    #[view]
    fn validateName(&self, name: Vec<u8>) -> Result<(), SCError> {
        name_util::validate_name(&name.as_slice())
    }

}
