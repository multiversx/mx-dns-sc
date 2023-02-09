multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopEncode, TopDecode, PartialEq, TypeAbi)]
pub enum UsernameStatus<M: ManagedTypeApi> {
    Available,
    PendingAccept { user: ManagedAddress<M> },
    PendingRegistration { user: ManagedAddress<M> },
    Registered { user: ManagedAddress<M> },
}
