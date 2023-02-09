multiversx_sc::imports!();

#[multiversx_sc::proxy]
pub trait UserBuiltin {
    #[endpoint(SetUserName)]
    fn set_user_name(&self, name: &ManagedBuffer);

    #[deprecated]
    #[endpoint(migrateUserName)]
    fn migrate_user_name(&self, name: &ManagedBuffer);

    #[endpoint(saveUserName)]
    fn save_user_name(&self, name: &ManagedBuffer);

    #[endpoint(deleteUserName)]
    fn delete_user_name(&self, name: &ManagedBuffer);
}
