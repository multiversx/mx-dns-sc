
#![no_std]
#![no_main]
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
    #[callback(set_user_name_callback)]
    fn SetUserName(&self, 
        #[callback_arg] cb_name_hash: &H256,
        name_hash: &[u8]);
}

#[elrond_wasm_derive::contract(DnsImpl)]
pub trait Dns {

    #[init]
    fn init(&self, registration_cost: &BigUint) {
        self.set_registration_cost(registration_cost);
    }

    #[payable]
    #[endpoint]
    fn register(&self,
            name: Vec<u8>,
            #[payment] payment: BigUint) -> SCResult<()>  {

        sc_try!(self.check_feature_on(&b"register"[..], true));

        sc_try!(name_validation::validate_name(&name.as_slice()));

        if payment != self.get_registration_cost() {
            return sc_error!("should pay exactly the registration cost");
        }

        let name_hash = self.name_hash(&name);
        if shard_id(&name_hash) != self.get_own_shard_id() {
            return sc_error!("name belongs to another shard");
        }

        let vs = self.get_value_state(&name_hash);
        if !vs.is_available() {
            return sc_error!("name already taken");
        }

        let address = self.get_caller();
        self.set_value_state(&name_hash, &ValueState::Pending(address.clone()));

        let user_proxy = contract_proxy!(self, &address, User);
        user_proxy.SetUserName(
            &name_hash,
            &name);

        return Ok(())
    }

    #[callback]
    fn set_user_name_callback(&self, 
            result: AsyncCallResult<()>,
            #[callback_arg] cb_name_hash: H256) {

        match result {
            AsyncCallResult::Ok(()) => {
                // commit
                let vm = self.get_value_state(&cb_name_hash);
                if let ValueState::Pending(addr) = vm {
                    self.set_value_state(&cb_name_hash, &ValueState::Committed(addr));
                } else {
                    self.set_value_state(&cb_name_hash, &ValueState::None);
                }
            },
            AsyncCallResult::Err(_) => {
                // revert
                self.set_value_state(&cb_name_hash, &ValueState::None);
            }
        }
    }

    #[view]
    fn resolve(&self, name: Vec<u8>) -> OptionalResult<Address> {
        let name_hash = self.name_hash(&name);
        self.resolve_from_hash(name_hash) 
    }

    #[view(resolveFromHash)]
    fn resolve_from_hash(&self, name_hash: H256) -> OptionalResult<Address> {
        if shard_id(&name_hash) != self.get_own_shard_id() {
            return OptionalResult::None;
        }

        let vs = self.get_value_state(&name_hash);
        match vs {
            ValueState::Committed(address) => OptionalResult::Some(address),
            _ => OptionalResult::None
        }
    }

    #[endpoint]
    fn claim(&self) -> SCResult<()>  {
        let contract_owner = self.get_owner_address();
        if &self.get_caller() != &contract_owner {
            return sc_error!("only owner can claim");
        }

        self.send_tx(&contract_owner, &self.get_sc_balance(), "dns claim");

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
        shard_id(&self.get_sc_address())
    }

    #[view(nameHash)]
    fn name_hash(&self, name: &Vec<u8>) -> H256 {
        self.keccak256(&name).into()
    }

    #[view(nameShard)]
    fn name_shard(&self, name: Vec<u8>) -> u8 {
        shard_id(&self.name_hash(&name))
    }
 
    #[view(validateName)]
    fn validate_name(&self, name: Vec<u8>) -> SCResult<()> {
        name_validation::validate_name(&name.as_slice())
    }

    // FEATURES (TODO: extract to standard module)

    #[storage_get("feat:")]
    fn get_feature_flag(&self, feature_name: FeatureName) -> Option<bool>;

    #[storage_set("feat:")]
    fn set_feature_flag(&self, feature_name: FeatureName, value: Option<bool>);

    fn check_feature_on(&self, feature_name: &'static [u8], default: bool) -> SCResult<()> {
        if self.get_feature_flag(FeatureName(feature_name)).unwrap_or(default) {
            Ok(())
        } else {
            let mut msg = feature_name.to_vec();
            msg.extend_from_slice(&b" currently disabled"[..]);
            SCResult::Err(SCError::Dynamic(msg))
        }
    }

    #[endpoint(setFeatureFlag)]
    fn set_feature_flag_endpoint(&self, feature_name: Vec<u8>, value: bool) -> SCResult<()> {
        if self.get_caller() != self.get_owner_address() {
            return sc_error!("only owner allowed to change features");
        }
        self.set_feature_flag(FeatureName(feature_name.as_slice()), Some(value));
        Ok(())
    }

    // METADATA

    #[view]
    fn version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

}

pub struct FeatureName<'a>(&'a [u8]);

use elrond_wasm::elrond_codec::*;
impl<'a> Encode for FeatureName<'a> {
    #[inline]
    fn dep_encode_to<O: Output>(&self, dest: &mut O) -> Result<(), EncodeError> {
        dest.write(&self.0[..]);
        Result::Ok(())
    }
}
