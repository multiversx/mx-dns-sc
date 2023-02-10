#![no_std]

use base_usernames::ProxyTrait as _;
use common::name_split;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

static ELROND_SUFFIX: &[u8] = b"elrond";
static MULTIVERSX_SUFFIX: &[u8] = b"multiversX";

#[multiversx_sc::contract]
pub trait DomainManagerX {
    #[init]
    fn init(&self, base_usernames_address: ManagedAddress, registration_fee: BigUint) {
        self.base_usernames_address().set(base_usernames_address);
        self.registration_fee().set(registration_fee);
    }

    #[payable("EGLD")]
    #[endpoint]
    fn save(&self, username_multiversx: ManagedBuffer) {
        let (prefix, domain) = name_split::get_prefix_domain(&username_multiversx);
        let multiversx_suffix = ManagedBuffer::from(MULTIVERSX_SUFFIX);
        require!(domain == multiversx_suffix, "Domain must be .multiversX");

        // TODO: username extra validation (if there are any extra rules not covered by base-usernames)

        let username_elrond = name_split::concat(prefix, ELROND_SUFFIX);
        let is_available: bool = self
            .base()
            .is_available(username_elrond.clone())
            .execute_on_dest_context();
        require!(is_available, "Username not available (.elrond is taken)");

        let egld_payment = self.call_value().egld_value();
        let registration_fee = self.registration_fee().get();
        require!(
            egld_payment == registration_fee,
            "Wrong registration fee amount"
        );

        let user = self.blockchain().get_caller();
        self.base()
            .save(username_elrond, user)
            .async_call()
            .call_and_exit();
    }

    #[endpoint]
    fn migrate(&self, username_elrond: ManagedBuffer) {
        let (prefix, domain) = name_split::get_prefix_domain(&username_elrond);
        let elrond_suffix = ManagedBuffer::from(ELROND_SUFFIX);
        require!(domain == elrond_suffix, "Domain must be .elrond");

        let user_opt: OptionalValue<ManagedAddress> = self
            .base()
            .resolve(username_elrond.clone())
            .execute_on_dest_context();

        let user: ManagedAddress = user_opt
            .into_option()
            .unwrap_or_else(|| sc_panic!("Username not found"));

        let username_multiversx = name_split::concat(prefix, MULTIVERSX_SUFFIX);
        self.base()
            .delete(username_elrond)
            .async_call()
            .with_callback(
                self.callbacks()
                    .migrate_callback_save(username_multiversx, user),
            )
            .call_and_exit();
    }

    #[callback]
    fn migrate_callback_save(
        &self,
        username_multiversx: ManagedBuffer,
        user: ManagedAddress,
        #[call_result] result: ManagedAsyncCallResult<u64>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(_) => {
                self.base()
                    .save(username_multiversx.clone(), user)
                    .async_call()
                    .with_callback(self.callbacks().migrate_callback_write(username_multiversx))
                    .call_and_exit();
            }
            ManagedAsyncCallResult::Err(_) => {
                // TODO: error handling
            }
        }
    }

    #[callback]
    fn migrate_callback_write(
        &self,
        username_multiversx: ManagedBuffer,
        #[call_result] result: ManagedAsyncCallResult<u64>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(_) => {
                self.base()
                    .write(username_multiversx)
                    .async_call()
                    .call_and_exit();
            }
            ManagedAsyncCallResult::Err(_) => {
                // TODO: error handling
            }
        }
    }

    fn base(&self) -> base_usernames::Proxy<Self::Api> {
        let base_usernames_address = self.base_usernames_address().get();
        self.base_usernames(base_usernames_address)
    }

    #[view(getBaseUsernamesAddress)]
    #[storage_mapper("baseUsernamesAddress")]
    fn base_usernames_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("registrationFee")]
    fn registration_fee(&self) -> SingleValueMapper<BigUint>;

    #[proxy]
    fn base_usernames(&self, address: ManagedAddress) -> base_usernames::Proxy<Self::Api>;
}
