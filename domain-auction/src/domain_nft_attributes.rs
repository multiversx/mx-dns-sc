multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopEncode, TopDecode)]
pub struct DomainNftAttributes<M: ManagedTypeApi> {
    pub domain: ManagedBuffer<M>,
    pub expiration: u64,
}
