
#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(unused_attributes)]

mod name_util;

pub static OWNER_KEY:      [u8; 32] = [0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ];
pub static SHARD_BITS_KEY: [u8; 32] = [0x01, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ];
pub static SHARD_ID_KEY:   [u8; 32] = [0x02, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ];
pub static SIBLING_ADDR_PREFIX: u8 = 3;

// constructs keys for user data
pub fn sibling_addr_key(shard_id: u8) -> StorageKey {
    let mut key = [0u8; 32];
    key[0] = SIBLING_ADDR_PREFIX;
    key[31] = shard_id;
    key.into()
}

#[elrond_wasm_derive::callable(SiblingProxy)]
pub trait Sibling {
    fn register(&self, name: Vec<u8>, address: Address);
}

#[elrond_wasm_derive::contract(DnsImpl)]
pub trait Dns {

    fn init(&self, shard_id_bits: i64, shard_id: u8) -> Result<(), &str> {
        let owner = self.get_caller();
        self.storage_store_bytes32(&OWNER_KEY.into(), &owner.as_fixed_bytes());

        // save shard ids length
        if shard_id_bits <= 0 || shard_id_bits > 8 {
            return Err("shard_id_bits out of range");
        }
        self.storage_store_i64(&SHARD_BITS_KEY.into(), shard_id_bits);

        // save shard id
        if shard_id >= (1u8 << shard_id_bits) {
            return Err("shard_id out of range");
        }
        self.storage_store_i64(&SHARD_ID_KEY.into(), shard_id as i64);

        Ok(())
    }

    fn setSibling(&self, shard_id: u8, address: Address) -> Result<(), &str> {
        if self.get_caller() != self.getContractOwner() {
            return Err("only owner can set sibling"); 
        }

        if shard_id >= (1u8 << self.getShardIdBits()) {
            return Err("sibling shard_id out of range");
        }

        if shard_id == self.getOwnShardId() {
            return Err("sibling shard_id identical to self");
        }

        self.storage_store_bytes32(&sibling_addr_key(shard_id), &address.as_fixed_bytes());

        Ok(())
    }

    fn register(&self, name: Vec<u8>, address: Address) -> Result<(), &str>  {
        name_util::validate_name(&name)?;

        let (name_hash, name_shard_id) = self.name_hash_shard(&name);
        if name_shard_id == self.getOwnShardId() {
            self.storage_store_bytes32(&name_hash.into(), &address.as_fixed_bytes());
            return Ok(())
        }

        let sibling_addr = self.storage_load_bytes32(&sibling_addr_key(name_shard_id));
        if sibling_addr == [0u8; 32] {
            return Err("missing sibling contract");
        }

        let sibling_contract = contract_proxy!(self, &sibling_addr.into(), Sibling);
        sibling_contract.register(name, address);

        Ok(())
    }

    #[private]
    fn name_hash_shard(&self, name: &Vec<u8>) -> ([u8; 32], u8) {
        let name_hash = self.keccak256(&name);
        let shard = name_hash[31] & self.getShardMask();
        (name_hash, shard)
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
        self.storage_load_i64(&SHARD_ID_KEY.into()).unwrap() as u8
    }

    #[view]
    fn name_shard(&self, name: Vec<u8>) -> u8 {
        self.name_hash_shard(&name).1
    }

}
