#![no_std]
#![allow(clippy::string_lit_as_bytes)]
#![allow(clippy::ptr_arg)]

pub mod name_validation;
pub mod user_builtin;
pub mod value_state;

use value_state::*;

elrond_wasm::imports!();

type NameHash<M> = ManagedByteArray<M, 32>;

/// There are 256 DNS contracts deployed, each one processes some of the addresses.
///
/// The last byte of each address is the contract (or sibling) id, and they go from 0 to 255.
///
/// The contracts get split among shards, but there is no direct correspondence between the id and the shard id.
#[inline]
fn sibling_id(addr_bytes: &[u8; 32]) -> u8 {
    addr_bytes[31]
}

#[elrond_wasm::derive::contract]
pub trait Dns: elrond_wasm_modules::features::FeaturesModule {
    #[proxy]
    fn user_builtin_proxy(&self, to: ManagedAddress) -> user_builtin::Proxy<Self::Api>;

    #[init]
    fn init(&self, registration_cost: &BigUint) {
        self.set_registration_cost(registration_cost);
    }

    fn validate_name_shard(&self, name_hash: &NameHash<Self::Api>) {
        require!(
            sibling_id(&name_hash.to_byte_array()) == self.get_own_sibling_id(),
            "name belongs to another shard"
        );
    }

    /// `name_hash` is redundant, but passed to the method so we don't compute it twice.
    fn validate_register_input(&self, name: &ManagedBuffer, name_hash: &NameHash<Self::Api>) {
        self.check_feature_on(b"register", true);

        self.validate_name(name);

        self.validate_name_shard(name_hash);

        let vs = self.get_value_state(name_hash);
        require!(vs.is_available(), "name already taken");
    }

    #[view(canRegister)]
    fn can_register(&self, name: ManagedBuffer) {
        let name_hash = self.name_hash(&name);
        self.validate_register_input(&name, &name_hash)
    }

    #[payable("EGLD")]
    #[endpoint]
    fn register(&self, name: ManagedBuffer) {
        let payment = self.call_value().egld_value();
        let name_hash = self.name_hash(&name);
        self.validate_register_input(&name, &name_hash);

        require!(
            payment == self.get_registration_cost(),
            "should pay exactly the registration cost"
        );

        let address = self.blockchain().get_caller();
        self.set_value_state(&name_hash, &ValueState::Pending(address.clone()));

        self.user_builtin_proxy(address)
            .set_user_name(&name)
            .async_call()
            .with_callback(self.callbacks().set_user_name_callback(&name_hash))
            .call_and_exit()
    }

    #[callback]
    fn set_user_name_callback(
        &self,
        cb_name_hash: &NameHash<Self::Api>,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                // commit
                let vm = self.get_value_state(cb_name_hash);
                if let ValueState::Pending(addr) = vm {
                    self.set_value_state(cb_name_hash, &ValueState::Committed(addr));
                } else {
                    self.set_value_state(cb_name_hash, &ValueState::None);
                }
            }
            ManagedAsyncCallResult::Err(_) => {
                // revert
                self.set_value_state(cb_name_hash, &ValueState::None);
            }
        }
    }

    #[view]
    fn resolve(&self, name: &ManagedBuffer) -> OptionalValue<ManagedAddress> {
        let name_hash = self.name_hash(name);
        self.resolve_from_hash(name_hash)
    }

    #[view(resolveFromHash)]
    fn resolve_from_hash(&self, name_hash: NameHash<Self::Api>) -> OptionalValue<ManagedAddress> {
        if sibling_id(&name_hash.to_byte_array()) != self.get_own_sibling_id() {
            return OptionalValue::None;
        }

        let vs = self.get_value_state(&name_hash);
        if let ValueState::Committed(address) = vs {
            OptionalValue::Some(address)
        } else {
            OptionalValue::None
        }
    }

    #[view(checkPending)]
    fn check_pending(&self, name: &ManagedBuffer) -> OptionalValue<ManagedAddress> {
        let name_hash = self.name_hash(name);
        if sibling_id(&name_hash.to_byte_array()) != self.get_own_sibling_id() {
            return OptionalValue::None;
        }

        let vs = self.get_value_state(&name_hash);
        if let ValueState::Pending(address) = vs {
            OptionalValue::Some(address)
        } else {
            OptionalValue::None
        }
    }

    #[only_owner]
    #[view(resetPending)]
    fn reset_pending(&self, name: &ManagedBuffer) {
        let name_hash = self.name_hash(name);
        self.validate_name_shard(&name_hash);

        let vs = self.get_value_state(&name_hash);
        if let ValueState::Pending(_) = vs {
            self.set_value_state(&name_hash, &ValueState::None);
        }
    }

    #[only_owner]
    #[endpoint]
    fn claim(&self) {
        self.send().direct_egld(
            &self.blockchain().get_caller(),
            &self
                .blockchain()
                .get_sc_balance(&EgldOrEsdtTokenIdentifier::egld(), 0),
            b"dns claim",
        );
    }

    // STORAGE

    #[view(getRegistrationCost)]
    #[storage_get("registration_cost")]
    fn get_registration_cost(&self) -> BigUint;

    #[storage_set("registration_cost")]
    fn set_registration_cost(&self, registration_cost: &BigUint);

    #[storage_get("value_state")]
    fn get_value_state(&self, name_hash: &NameHash<Self::Api>) -> ValueState<Self::Api>;

    #[storage_set("value_state")]
    fn set_value_state(&self, name_hash: &NameHash<Self::Api>, value_state: &ValueState<Self::Api>);

    // UTILS

    #[view(getContractOwner)]
    fn get_owner_address_endpoint(&self) -> ManagedAddress {
        self.blockchain().get_owner_address()
    }

    /// Incorrectly named, it should actually read the `sibling id` or `contract id`.
    #[view(getOwnShardId)]
    fn get_own_sibling_id(&self) -> u8 {
        sibling_id(&self.blockchain().get_sc_address().to_byte_array())
    }

    #[view(nameHash)]
    fn name_hash(&self, name: &ManagedBuffer) -> NameHash<Self::Api> {
        self.crypto().keccak256(name)
    }

    /// Incorrectly named, it is the contract id that corresponds to the given name, from 0 to 255.
    #[view(nameShard)]
    fn name_shard(&self, name: &ManagedBuffer) -> u8 {
        sibling_id(&self.name_hash(name).to_byte_array())
    }

    #[view(validateName)]
    fn validate_name(&self, name: &ManagedBuffer) {
        name_validation::validate_name(name).unwrap_or_else(|err| sc_panic!(err));
    }

    // METADATA

    #[view]
    fn version(&self) -> &'static [u8] {
        env!("CARGO_PKG_VERSION").as_bytes()
    }
}
