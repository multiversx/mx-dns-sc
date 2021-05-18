#![no_std]
#![allow(unused_attributes)]
#![allow(non_snake_case)]
#![allow(clippy::string_lit_as_bytes)]
#![allow(clippy::ptr_arg)]

#[cfg(feature = "elrond-wasm-module-features-default")]
pub use elrond_wasm_module_features_default as features;
#[cfg(feature = "elrond-wasm-module-features-wasm")]
pub use elrond_wasm_module_features_wasm as features;

pub mod name_validation;
pub mod user_builtin;
pub mod value_state;

use features::feature_guard;
use value_state::*;

elrond_wasm::imports!();

#[inline]
fn shard_id(addr: &H256) -> u8 {
    addr.as_bytes()[31]
}

#[elrond_wasm_derive::contract]
pub trait Dns: features::FeaturesModule {
    #[proxy]
    fn user_builtin_proxy(&self, to: Address) -> user_builtin::Proxy<Self::SendApi>;

    #[init]
    fn init(&self, registration_cost: &Self::BigUint) {
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
        feature_guard!(self, b"register", true);

        name_validation::validate_name(name.as_slice())?;

        self.validate_name_shard(name_hash)?;

        let vs = self.get_value_state(&name_hash);
        require!(vs.is_available(), "name already taken");

        Ok(())
    }

    #[view(canRegister)]
    fn can_register(&self, name: BoxedBytes) -> SCResult<()> {
        let name_hash = self.name_hash(name.as_slice());
        self.validate_register_input(&name, &name_hash)
    }

    #[payable("EGLD")]
    #[endpoint]
    fn register(
        &self,
        name: BoxedBytes,
        #[payment] payment: Self::BigUint,
    ) -> SCResult<AsyncCall<Self::SendApi>> {
        let name_hash = self.name_hash(name.as_slice());
        self.validate_register_input(&name, &name_hash)?;

        require!(
            payment == self.get_registration_cost(),
            "should pay exactly the registration cost"
        );

        let address = self.blockchain().get_caller();
        self.set_value_state(&name_hash, &ValueState::Pending(address.clone()));

        Ok(self
            .user_builtin_proxy(address)
            .set_user_name(&name)
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
        self.validate_name_shard(&name_hash)?;

        let vs = self.get_value_state(&name_hash);
        if let ValueState::Pending(_) = vs {
            self.set_value_state(&name_hash, &ValueState::None);
        }

        Ok(())
    }

    #[endpoint]
    fn claim(&self) -> SCResult<()> {
        only_owner!(self, "only owner can claim");

        self.send().direct_egld(
            &self.blockchain().get_caller(),
            &self.blockchain().get_sc_balance(),
            b"dns claim",
        );

        Ok(())
    }

    // STORAGE

    #[view(getRegistrationCost)]
    #[storage_get("registration_cost")]
    fn get_registration_cost(&self) -> Self::BigUint;

    #[storage_set("registration_cost")]
    fn set_registration_cost(&self, registration_cost: &Self::BigUint);

    #[storage_get("value_state")]
    fn get_value_state(&self, name_hash: &H256) -> ValueState;

    #[storage_set("value_state")]
    fn set_value_state(&self, name_hash: &H256, value_state: &ValueState);

    // UTILS

    #[view(getContractOwner)]
    fn get_owner_address_endpoint(&self) -> Address {
        self.blockchain().get_owner_address()
    }

    #[view(getOwnShardId)]
    fn get_own_shard_id(&self) -> u8 {
        shard_id(&self.blockchain().get_sc_address().into())
    }

    #[view(nameHash)]
    fn name_hash(&self, name: &[u8]) -> H256 {
        self.crypto().keccak256(name)
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
