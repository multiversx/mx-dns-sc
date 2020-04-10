
#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(unused_attributes)]

pub mod name_util;
pub mod storage;

use storage::*;

fn shard_id(addr: &Address, shard_mask: u8) -> u8 {
    let last_byte = addr.as_bytes()[31];
    last_byte & shard_mask
}

#[elrond_wasm_derive::callable(SiblingProxy)]
pub trait Sibling {
    fn register(&self, name: Vec<u8>, address: Address);
}

#[elrond_wasm_derive::callable(UserProxy)]
pub trait User {
    #[callback(userStoreCallback)]
    fn storeIfNotExists(&self,
        #[callback_arg] name_hash: &H256,
        key: &StorageKey,
        name: &Vec<u8>);
}

#[elrond_wasm_derive::contract(DnsImpl)]
pub trait Dns {

    fn init(&self,
            user_storage_key: StorageKey,
            shard_id_bits: u8
        ) -> Result<(), &str> {
        
        let owner = self.get_caller();
        self.storage_store_bytes32(&OWNER_KEY.into(), &owner.as_fixed_bytes());
        self.storage_store_bytes32(&USER_STORAGE_KEY_KEY.into(), &user_storage_key.as_fixed_bytes());

        // save shard ids length
        if shard_id_bits <= 0 || shard_id_bits > 8 {
            return Err("shard_id_bits out of range");
        }
        self.storage_store_i64(&SHARD_BITS_KEY.into(), shard_id_bits as i64);

        Ok(())
    }

    fn setSiblings(&self,
            #[multi(self.getShardMask() as usize)]
            sibling_addresses: Vec<Address>
        ) -> Result<(), &str> {
        
        if self.get_caller() != self.getContractOwner() {
            return Err("only owner can set siblings"); 
        }

        // save siblings
        let own_shard_id = self.getOwnShardId();
        for sibl_addr in sibling_addresses.iter() {
            let sibl_shard_id = shard_id(&sibl_addr, self.getShardMask());
            if sibl_shard_id == own_shard_id {
                return Err("sibling has the same shard id as self");
            }
            let sibl_key = sibling_addr_key(sibl_shard_id);
            if self.storage_load_len(&sibl_key) > 0 {
                return Err("2 siblings have the same shard id");
            }

            self.storage_store_bytes32(&sibl_key, &sibl_addr.as_fixed_bytes());
        }

        Ok(())
    }

    /// Returns: 0 if registration happened ok, 1 if sent to another shard, 2 if failed
    fn register(&self, name: Vec<u8>, address: Address) -> Result<i32, &str>  {
        name_util::validate_name(&name.as_slice())?;

        let (name_hash, name_shard_id) = self.name_hash_shard(&name);
        if name_shard_id == self.getOwnShardId() {
            let vs = self.load_value_state(&name_hash);
            if !vs.is_available() {
                return Ok(2);
            }

            self.store_value_state(&name_hash, &ValueState::Pending(address.clone()));

            let user_proxy = contract_proxy!(self, &address, User);
            user_proxy.storeIfNotExists(
                &name_hash,
                &self.getUserStorageKey(),
                &name);

            return Ok(0)
        }

        let sibl_key = &sibling_addr_key(name_shard_id);
        if self.storage_load_len(&sibl_key) == 0 {
            return Err("missing sibling contract");
        } 
        let sibling_addr = self.storage_load_bytes32(&sibl_key);
        let sibling_contract = contract_proxy!(self, &sibling_addr.into(), Sibling);
        sibling_contract.register(name, address);

        Ok(1)
    }

    #[callback]
    fn userStoreCallback(&self, 
            success: bool,
            #[callback_arg] name_hash: H256) {

        if success {
            // commit
            let vm = self.load_value_state(&name_hash);
            if let ValueState::Pending(addr) = vm {
                self.store_value_state(&name_hash, &ValueState::Committed(addr));
            }
        } else {
            // revert
            self.store_value_state(&name_hash, &ValueState::None);
        }
    }

    fn resolve(&self, name: Vec<u8>) -> Option<Address> {
        let (name_hash, name_shard_id) = self.name_hash_shard(&name);
        if name_shard_id != self.getOwnShardId() {
            return None;
        }

        let vs = self.load_value_state(&name_hash);
        match vs {
            ValueState::Committed(address) => Some(address),
            _ => None
        }
    }

    #[private]
    fn load_value_state(&self, key: &StorageKey) -> ValueState {
        let value_raw = self.storage_load(&key);
        ValueState::from_bytes(value_raw.as_slice())
    }

    #[private]
    fn store_value_state(&self, key: &StorageKey, vs: &ValueState) {
        self.storage_store(&key, &vs.to_bytes());
    }

    #[private]
    fn name_hash_shard(&self, name: &Vec<u8>) -> (StorageKey, u8) {
        let name_hash = self.keccak256(&name);
        let shard_mask = self.getShardMask();
        let shard = name_hash[31] & shard_mask;
        (name_hash.into(), shard)
    }

    #[view]
    fn getContractOwner(&self) -> Address {
        self.storage_load_bytes32(&OWNER_KEY.into()).into()
    }

    #[view]
    fn getShardIdBits(&self) -> i64 {
        self.storage_load_i64(&SHARD_BITS_KEY.into()).unwrap()
    }

    #[private]
    fn getShardMask(&self) -> u8 {
        (1 << self.getShardIdBits()) - 1
    }

    #[view]
    fn getOwnShardId(&self) -> u8 {
        shard_id(&self.get_own_address(), self.getShardMask())
    }

    #[view]
    fn getUserStorageKey(&self) -> StorageKey {
        self.storage_load_bytes32(&USER_STORAGE_KEY_KEY.into()).into()
    }

    #[view]
    fn nameShard(&self, name: Vec<u8>) -> u8 {
        self.name_hash_shard(&name).1
    }

    #[view]
    fn validateName(&self, name: Vec<u8>) -> Result<(), &str> {
        name_util::validate_name(&name.as_slice())
    }

}
