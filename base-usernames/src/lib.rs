#![no_std]

mod username_status;

use common::name_hash::NameHash;
use common::name_split;
use common::user_builtin;
use username_status::UsernameStatus;

use multiversx_sc_modules::only_admin;
use multiversx_sc_modules::pause;

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait BaseUsernames: only_admin::OnlyAdminModule + pause::PauseModule {
    #[init]
    fn init(&self, save_gas_limit: u64, delete_gas_limit: u64) {
        self.save_gas_limit().set(save_gas_limit);
        self.delete_gas_limit().set(delete_gas_limit);
    }

    #[endpoint(setDomainManager)]
    fn set_domain_manager(&self, domain: ManagedBuffer, manager: ManagedAddress) {
        self.require_caller_is_admin();
        let hash = self.hash(&domain);
        self.domain_manager(hash).set(manager);
    }

    #[endpoint(unSetDomainManager)]
    fn unset_domain_manager(&self, domain: ManagedBuffer) {
        self.require_caller_is_admin();
        let hash = self.hash(&domain);
        self.domain_manager(hash).clear();
    }

    #[endpoint]
    fn save(&self, username: ManagedBuffer, user: ManagedAddress) {
        let domain = name_split::get_domain(&username);
        self.require_caller_is_domain_manager(domain);

        let hash = self.hash(&username);
        self.username_status(hash).update(|status| {
            require!(
                status == &UsernameStatus::Available,
                "Username already taken"
            );
            *status = UsernameStatus::PendingAccept { user };
        });

        // TODO: replace with self.blockchain().get_original_caller() when it's added to the rust framework
        let original_caller = ManagedAddress::zero();
        let caller = self.blockchain().get_caller();
        if caller == original_caller {
            self.accept(username);
        }
    }

    #[endpoint]
    fn accept(&self, username: ManagedBuffer) {
        let hash = self.hash(&username);
        let user = self.username_status(hash).update(|status| {
            let user = if let UsernameStatus::PendingAccept { user } = status {
                user.clone()
            } else {
                sc_panic!("Wrong status");
            };

            let caller = self.blockchain().get_caller();
            require!(caller == user, "Caller is not the pending user");
            *status = UsernameStatus::Registered { user: caller };

            user
        });

        let save_gas_limit = self.save_gas_limit().get();

        self.user_builtin_proxy(user)
            .delete_user_name(username)
            .with_gas_limit(save_gas_limit)
            .async_call()
            .call_and_exit();
    }

    #[endpoint]
    fn delete(&self, username: ManagedBuffer) {
        let domain = name_split::get_domain(&username);
        self.require_caller_is_domain_manager(domain);

        let delete_gas_limit = self.delete_gas_limit().get();
        let hash = self.hash(&username);
        let status = self.username_status(hash).take();
        if let UsernameStatus::Registered { user } = status {
            self.user_builtin_proxy(user)
                .delete_user_name(username)
                .with_gas_limit(delete_gas_limit)
                .async_call()
                .call_and_exit();
        }
    }

    #[endpoint]
    fn resolve(&self, username: ManagedBuffer) -> OptionalValue<ManagedAddress> {
        let hash = self.hash(&username);
        if let UsernameStatus::Registered { user } = self.username_status(hash).get() {
            OptionalValue::Some(user)
        } else {
            OptionalValue::None
        }
    }

    fn require_caller_is_domain_manager(&self, domain: ManagedBuffer) {
        let hash = self.hash(&domain);
        let caller = self.blockchain().get_caller();
        require!(
            self.domain_manager(hash).get() == caller,
            "Caller is not the domain manager"
        );
    }

    fn hash(&self, username: &ManagedBuffer) -> NameHash<Self::Api> {
        self.crypto().keccak256(username)
    }

    #[storage_mapper("domainManager")]
    fn domain_manager(&self, hash: NameHash<Self::Api>) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("status")]
    fn username_status(
        &self,
        hash: NameHash<Self::Api>,
    ) -> SingleValueMapper<UsernameStatus<Self::Api>>;

    #[storage_mapper("saveGasLimit")]
    fn save_gas_limit(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("deleteGasLimit")]
    fn delete_gas_limit(&self) -> SingleValueMapper<u64>;

    #[proxy]
    fn user_builtin_proxy(&self, to: ManagedAddress) -> user_builtin::Proxy<Self::Api>;
}
