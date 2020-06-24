
#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(unused_attributes)]

pub mod name_validation;
pub mod value_state;

use value_state::*;

imports!();

#[inline]
fn shard_id(addr: &Address) -> u8 {
    addr.as_bytes()[31]
}

#[elrond_wasm_derive::callable(UserProxy)]
pub trait User {
    #[callback(setUserNameCallback)]
    fn SetUserName(&self, 
        #[callback_arg] cb_name_hash: &H256,
        name_hash: &H256);
}

#[elrond_wasm_derive::contract(DnsImpl)]
pub trait Dns {

    fn init(&self, registration_cost: &BigUint) {
        self._set_registration_cost(registration_cost);
    }

    #[payable]
    fn register(&self,
            name: Vec<u8>,
            #[payment] payment: BigUint) -> Result<(), SCError>  {

        name_validation::validate_name(&name.as_slice())?;

        if payment != self.getRegistrationCost() {
            return sc_error!("should pay exactly the registration cost");
        }

        let name_hash = self.nameHash(&name);
        if shard_id(&name_hash) != self.getOwnShardId() {
            return sc_error!("name belongs to another shard");
        }

        let vs = self._get_value_state(&name_hash);
        if !vs.is_available() {
            return sc_error!("name already taken");
        }

        let address = self.get_caller();
        self._set_value_state(&name_hash, &ValueState::Pending(address.clone()));

        let user_proxy = contract_proxy!(self, &address, User);
        user_proxy.SetUserName(
            &name_hash,
            &name_hash);

        return Ok(())
    }

    #[callback]
    fn setUserNameCallback(&self, 
            result: AsyncCallResult<()>,
            #[callback_arg] cb_name_hash: H256) {

        match result {
            AsyncCallResult::Ok(()) => {
                // commit
                let vm = self._get_value_state(&cb_name_hash);
                if let ValueState::Pending(addr) = vm {
                    self._set_value_state(&cb_name_hash, &ValueState::Committed(addr));
                } else {
                    self._set_value_state(&cb_name_hash, &ValueState::None);
                }
            },
            AsyncCallResult::Err(_) => {
                // revert
                self._set_value_state(&cb_name_hash, &ValueState::None);
            }
        }
        
    }

    fn resolve(&self, name: Vec<u8>) -> OptionalResult<Address> {
        let name_hash = self.nameHash(&name);
        if shard_id(&name_hash) != self.getOwnShardId() {
            return OptionalResult::None;
        }

        let vs = self._get_value_state(&name_hash);
        match vs {
            ValueState::Committed(address) => OptionalResult::Some(address),
            _ => OptionalResult::None
        }
    }

    fn claim(&self) -> Result<(), SCError>  {
        let contract_owner = self.getContractOwner();
        if &self.get_caller() != &contract_owner {
            return sc_error!("only owner can claim");
        }

        self.send_tx(&contract_owner, &self.get_sc_balance(), "dns claim");

        Ok(())
    }

    // STORAGE

    #[view]
    #[storage_get("registration_cost")]
    fn getRegistrationCost(&self) -> BigUint;

    #[private]
    #[storage_set("registration_cost")]
    fn _set_registration_cost(&self, registration_cost: &BigUint);

    #[private]
    #[storage_get("value_state")]
    fn _get_value_state(&self, name_hash: &H256) -> ValueState;

    #[private]
    #[storage_set("value_state")]
    fn _set_value_state(&self, name_hash: &H256, value_state: &ValueState);

    // UTILS

    #[view]
    fn getContractOwner(&self) -> Address {
        self.get_owner_address()
    }

    #[view]
    fn getOwnShardId(&self) -> u8 {
        shard_id(&self.get_sc_address())
    }

    #[view]
    fn nameHash(&self, name: &Vec<u8>) -> H256 {
        self.keccak256(&name).into()
    }

    #[view]
    fn nameShard(&self, name: Vec<u8>) -> u8 {
        shard_id(&self.nameHash(&name))
    }
 
    #[view]
    fn validateName(&self, name: Vec<u8>) -> Result<(), SCError> {
        name_validation::validate_name(&name.as_slice())
    }

    // METADATA

    fn version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}
