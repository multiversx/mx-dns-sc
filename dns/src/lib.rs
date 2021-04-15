#![no_std]
#![allow(unused_attributes)]
#![allow(non_snake_case)]
#![allow(clippy::string_lit_as_bytes)]
#![allow(clippy::ptr_arg)]

#[cfg(feature = "elrond-wasm-module-features-default")]
pub use elrond_wasm_module_features_default as elrond_wasm_module_features;
#[cfg(feature = "elrond-wasm-module-features-wasm")]
pub use elrond_wasm_module_features_wasm as elrond_wasm_module_features;

pub mod name_validation;
pub mod value_state;

use elrond_wasm_module_features::*;
use value_state::*;

elrond_wasm::imports!();

#[inline]
fn shard_id(addr: &H256) -> u8 {
    addr.as_bytes()[31]
}

#[elrond_wasm_derive::callable(UserProxy)]
pub trait User {
    fn SetUserName(&self, name: &BoxedBytes) -> ContractCall<BigUint, ()>;
}

#[elrond_wasm_derive::contract(DnsImpl)]
pub trait Dns {
    #[module(FeaturesModuleImpl)]
    fn features_module(&self) -> FeaturesModuleImpl<T, BigInt, BigUint>;

    #[init]
    fn init(&self, registration_cost: &BigUint) {
        self.set_registration_cost(registration_cost);
    }

    fn validate_name_shard(&self, name_hash: &H256) -> SCResult<()> {
        require!(
            shard_id(&name_hash) == self.get_own_shard_id(),
            "name belongs to another shard"
        );
        Ok(())
    }

    /// `name_hash` is redundant, but passed to the method so we don't compute it twice.
    fn validate_register_input(&self, name: &BoxedBytes, name_hash: &H256) -> SCResult<()> {
        feature_guard!(self.features_module(), b"register", true);

        sc_try!(name_validation::validate_name(name.as_slice()));

        sc_try!(self.validate_name_shard(name_hash));

        let vs = self.get_value_state(&name_hash);
        require!(vs.is_available(), "name already taken");

        Ok(())
    }

    #[view(canRegister)]
    fn can_register(&self, name: BoxedBytes) -> SCResult<()> {
        let name_hash = self.name_hash(name.as_slice());
        self.validate_register_input(&name, &name_hash)
    }

    #[payable]
    #[endpoint]
    fn register(
        &self,
        name: BoxedBytes,
        #[payment] payment: BigUint,
    ) -> SCResult<AsyncCall<BigUint>> {
        let name_hash = self.name_hash(name.as_slice());
        sc_try!(self.validate_register_input(&name, &name_hash));

        require!(
            payment == self.get_registration_cost(),
            "should pay exactly the registration cost"
        );

        let address = self.get_caller();
        self.set_value_state(&name_hash, &ValueState::Pending(address.clone()));

        Ok(contract_call!(self, address, UserProxy)
            .SetUserName(&name)
            .async_call()
            .with_callback(self.callbacks().set_user_name_callback(&name_hash)))
    }

    #[callback]
    fn set_user_name_callback(
        &self,
        cb_name_hash: &H256,
        #[call_result] result: AsyncCallResult<()>,
    ) {
        match result {
            AsyncCallResult::Ok(()) => {
                // commit
                let vm = self.get_value_state(&cb_name_hash);
                if let ValueState::Pending(addr) = vm {
                    self.set_value_state(cb_name_hash, &ValueState::Committed(addr));
                } else {
                    self.set_value_state(cb_name_hash, &ValueState::None);
                }
            }
            AsyncCallResult::Err(_) => {
                // revert
                self.set_value_state(cb_name_hash, &ValueState::None);
            }
        }
    }

    #[view]
    fn resolve(&self, name: &[u8]) -> OptionalResult<Address> {
        let name_hash = self.name_hash(name);
        self.resolve_from_hash(name_hash)
    }

    #[view(resolveFromHash)]
    fn resolve_from_hash(&self, name_hash: H256) -> OptionalResult<Address> {
        if shard_id(&name_hash) != self.get_own_shard_id() {
            return OptionalResult::None;
        }

        let vs = self.get_value_state(&name_hash);
        if let ValueState::Committed(address) = vs {
            OptionalResult::Some(address)
        } else {
            OptionalResult::None
        }
    }

    #[view(checkPending)]
    fn check_pending(&self, name: &[u8]) -> OptionalResult<Address> {
        let name_hash = self.name_hash(name);
        if shard_id(&name_hash) != self.get_own_shard_id() {
            return OptionalResult::None;
        }

        let vs = self.get_value_state(&name_hash);
        if let ValueState::Pending(address) = vs {
            OptionalResult::Some(address)
        } else {
            OptionalResult::None
        }
    }

    #[view(resetPending)]
    fn reset_pending(&self, name: &[u8]) -> SCResult<()> {
        only_owner!(self, "only owner can resetPending");

        let name_hash = self.name_hash(name);
        sc_try!(self.validate_name_shard(&name_hash));

        let vs = self.get_value_state(&name_hash);
        if let ValueState::Pending(_) = vs {
            self.set_value_state(&name_hash, &ValueState::None);
        }

        Ok(())
    }

    #[endpoint]
    fn claim(&self) -> SCResult<()> {
        let contract_owner = self.get_owner_address();
        if self.get_caller() != contract_owner {
            return sc_error!("only owner can claim");
        }

        self.send()
            .direct_egld(&contract_owner, &self.get_sc_balance(), b"dns claim");

        Ok(())
    }

    // STORAGE

    #[view(getRegistrationCost)]
    #[storage_get("registration_cost")]
    fn get_registration_cost(&self) -> BigUint;

    #[storage_set("registration_cost")]
    fn set_registration_cost(&self, registration_cost: &BigUint);

    #[storage_get("value_state")]
    fn get_value_state(&self, name_hash: &H256) -> ValueState;

    #[storage_set("value_state")]
    fn set_value_state(&self, name_hash: &H256, value_state: &ValueState);

    // UTILS

    #[view(getContractOwner)]
    fn get_owner_address_endpoint(&self) -> Address {
        self.get_owner_address()
    }

    #[view(getOwnShardId)]
    fn get_own_shard_id(&self) -> u8 {
        shard_id(&self.get_sc_address().into())
    }

    #[view(nameHash)]
    fn name_hash(&self, name: &[u8]) -> H256 {
        self.keccak256(name)
    }

    #[view(nameShard)]
    fn name_shard(&self, name: &[u8]) -> u8 {
        shard_id(&self.name_hash(&name))
    }

    #[view(validateName)]
    fn validate_name(&self, name: &[u8]) -> SCResult<()> {
        name_validation::validate_name(name)
    }

    // METADATA

    #[view]
    fn version(&self) -> &'static [u8] {
        env!("CARGO_PKG_VERSION").as_bytes()
    }
}
