#![no_std]

pub mod name_validation;
pub mod user_builtin;
pub mod value_state;

use name_validation::SuffixType;
use value_state::ValueState;

multiversx_sc::imports!();

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

#[multiversx_sc::contract]
pub trait Dns: multiversx_sc_modules::features::FeaturesModule {
    #[proxy]
    fn user_builtin_proxy(&self, to: ManagedAddress) -> user_builtin::Proxy<Self::Api>;

    #[init]
    fn init(&self, registration_cost: &BigUint) {
        self.registration_cost().set(registration_cost);
    }

    fn validate_name_shard(&self, name_hash: &NameHash<Self::Api>) {
        require!(
            sibling_id(&name_hash.to_byte_array()) == self.get_own_sibling_id(),
            "name belongs to another shard"
        );
    }

    /// `name_hash` is redundant, but passed to the method so we don't compute it twice.
    fn validate_register_input(&self, name: &ManagedBuffer, name_hash: &NameHash<Self::Api>) {
        self.check_register_feature_on();

        self.validate_name(name);

        self.validate_name_shard(name_hash);

        let vs = self.value_state(name_hash).get();
        require!(vs.is_available(), "name already taken");
    }

    fn check_register_feature_on(&self) {
        self.check_feature_on(b"register", true);
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
            payment == self.registration_cost().get(),
            "should pay exactly the registration cost"
        );

        let address = self.blockchain().get_caller();
        self.value_state(&name_hash)
            .set(&ValueState::PendingX(address.clone()));

        let gas_limit = self.update_gas_limit().get();
        self.user_builtin_proxy(address)
            .set_user_name(&name)
            .with_gas_limit(gas_limit)
            .async_call()
            .with_callback(self.callbacks().update_value_state_callback(&name_hash))
            .call_and_exit()
    }

    #[endpoint]
    fn migrate(&self, name: ManagedBuffer) {
        let name_with_x_suffix = name_validation::prepare_and_validate_name_for_migration(&name)
            .unwrap_or_else(|err| sc_panic!(err));

        self.check_register_feature_on();

        let name_hash = self.unchecked_name_hash(&name);

        self.validate_name_shard(&name_hash);

        let address = self.value_state(&name_hash).update(|vs| {
            let address = vs.start_migration();

            let caller = self.blockchain().get_caller();
            require!(caller == address, "name not owned by the caller");
            address
        });

        let gas_limit = self.update_gas_limit().get();
        self.user_builtin_proxy(address)
            .migrate_user_name(&name_with_x_suffix)
            .with_gas_limit(gas_limit)
            .async_call()
            .with_callback(self.callbacks().update_value_state_callback(&name_hash))
            .call_and_exit();
    }

    #[callback]
    fn update_value_state_callback(
        &self,
        cb_name_hash: &NameHash<Self::Api>,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        self.value_state(cb_name_hash).update(|vs| {
            if result.is_ok() {
                vs.finalize();
            } else {
                vs.revert();
            }
        });
    }

    #[view]
    fn resolve(&self, name: &ManagedBuffer) -> OptionalValue<ManagedAddress> {
        let (name_hash, computed_suffix_type) = self.compute_name_hash_and_classify(name);
        self.resolve_from_hash(name_hash)
            .into_option()
            .and_then(|address_suffix_multi_value| {
                let (address, stored_suffix_type) = address_suffix_multi_value.into_tuple();
                if computed_suffix_type == stored_suffix_type {
                    Some(address)
                } else {
                    None
                }
            })
            .into()
    }

    #[view(resolveFromHash)]
    fn resolve_from_hash(
        &self,
        name_hash: NameHash<Self::Api>,
    ) -> OptionalValue<MultiValue2<ManagedAddress, SuffixType>> {
        if sibling_id(&name_hash.to_byte_array()) != self.get_own_sibling_id() {
            return OptionalValue::None;
        }

        let vs = self.value_state(&name_hash).get();
        match vs {
            ValueState::RegisteredX(address) => {
                OptionalValue::Some((address, SuffixType::X).into())
            }
            ValueState::RegisteredElrond(address) => {
                OptionalValue::Some((address, SuffixType::Elrond).into())
            }
            _ => OptionalValue::None,
        }
    }

    #[view(getNameValueState)]
    fn get_name_value_state(&self, name: &ManagedBuffer) -> ValueState<Self::Api> {
        let name_hash = self.name_hash(name);
        self.value_state(&name_hash).get()
    }

    #[only_owner]
    #[endpoint]
    fn claim(&self) {
        self.send().direct_egld(
            &self.blockchain().get_caller(),
            &self
                .blockchain()
                .get_sc_balance(&EgldOrEsdtTokenIdentifier::egld(), 0),
        );
    }

    #[only_owner]
    #[endpoint(setUpdateGasLimit)]
    fn set_update_gas_limit(&self, gas_limit: u64) {
        self.update_gas_limit().set(gas_limit);
    }

    // STORAGE

    #[view(getRegistrationCost)]
    #[storage_mapper("registration_cost")]
    fn registration_cost(&self) -> SingleValueMapper<BigUint>;

    #[view(getValueState)]
    #[storage_mapper("value_state")]
    fn value_state(
        &self,
        name_hash: &NameHash<Self::Api>,
    ) -> SingleValueMapper<ValueState<Self::Api>>;

    #[view(getUpdateGasLimit)]
    #[storage_mapper("update_gas_limit")]
    fn update_gas_limit(&self) -> SingleValueMapper<u64>;

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
        let (name_hash, _) = self.compute_name_hash_and_classify(name);
        name_hash
    }

    fn compute_name_hash_and_classify(
        &self,
        name: &ManagedBuffer,
    ) -> (NameHash<Self::Api>, SuffixType) {
        let (prepared_name, suffix_type) =
            name_validation::prepare_name_for_hash_and_classify(name);
        let name_hash = self.unchecked_name_hash(&prepared_name);
        (name_hash, suffix_type)
    }

    fn unchecked_name_hash(&self, name: &ManagedBuffer) -> NameHash<Self::Api> {
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
