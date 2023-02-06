multiversx_sc::imports!();

#[multiversx_sc::derive::proxy]
pub trait UserBuiltin {
    #[endpoint(SetUserName)]
    fn set_user_name(&self, name: &ManagedBuffer);

    #[endpoint(migrateUserName)]
    fn migrate_user_name(&self, name: &ManagedBuffer);
}
