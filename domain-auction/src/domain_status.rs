multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopEncode, TopDecode, PartialEq, TypeAbi)]
pub enum DomainStatus<M: ManagedTypeApi> {
    Available,
    StartingAuction,
    Auctioned {
        auction_id: u64,
    },
    Owned {
        owner: ManagedAddress<M>,
        nft_nonce: u64,
    },
    Nftized,
    ReservedByOwner,
}
