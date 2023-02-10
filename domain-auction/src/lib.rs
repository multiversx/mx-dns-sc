#![no_std]

mod domain_nft_attributes;
mod domain_status;
use base_usernames::ProxyTrait as _;
use domain_nft_attributes::DomainNftAttributes;
use domain_status::DomainStatus;
use esdt_nft_marketplace::auction::ProxyTrait as _;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::contract]
pub trait DomainAuction {
    #[init]
    fn init(
        &self,
        base_usernames_address: ManagedAddress,
        marketplace_address: ManagedAddress,
        nft_royalties: BigUint,
        domain_validity_period: u64,
        auction_min_bid: BigUint,
        auction_duration: u64,
        domain_token_id: TokenIdentifier,
        domain_token_name: ManagedBuffer,
        domain_blacklist: ManagedVec<ManagedBuffer>,
    ) {
        self.base_usernames_address().set(base_usernames_address);
        self.marketplace_address().set(marketplace_address);

        self.nft_royalties().set(nft_royalties);
        self.domain_validity_period().set(domain_validity_period);
        self.auction_min_bid().set(auction_min_bid);
        self.auction_duration().set(auction_duration);

        self.domain_token_id().set_token_id(domain_token_id);
        self.domain_token_name().set(domain_token_name);

        for domain in &domain_blacklist {
            self.domain_blacklist().insert(domain);
        }
    }

    #[endpoint(startAuction)]
    fn start_auction(&self, domain: ManagedBuffer) {
        let expiration = self.reserve_domain(&domain);
        let domain_nft = self.create_domain_nft(domain.clone(), expiration);

        let min_bid = self.auction_min_bid().get();
        let max_bid = BigUint::zero();
        let deadline = self.blockchain().get_block_timestamp() + self.auction_duration().get();

        let marketplace_address = self.marketplace_address().get();
        self.esdt_nft_marketplace(marketplace_address)
            .auction_token(
                min_bid,
                max_bid,
                deadline,
                EgldOrEsdtTokenIdentifier::egld(),
                OptionalValue::<BigUint>::None,
                OptionalValue::<bool>::None,
                OptionalValue::<u64>::None,
                OptionalValue::<u64>::None,
            )
            .with_esdt_transfer(domain_nft.clone())
            .async_call()
            .with_callback(self.callbacks().start_auction_callback(domain, domain_nft))
            .call_and_exit();
    }

    #[callback]
    fn start_auction_callback(
        &self,
        domain: ManagedBuffer,
        domain_nft: EsdtTokenPayment,
        #[call_result] result: ManagedAsyncCallResult<u64>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(auction_id) => {
                self.domain_status(&domain)
                    .update(|status| *status = DomainStatus::Auctioned { auction_id });
            }
            ManagedAsyncCallResult::Err(_) => {
                self.burn_domain_nft(domain_nft);
            }
        }
    }

    #[payable("*")]
    #[endpoint(claimDomain)]
    fn claim_domain(&self, domain_manager: ManagedAddress) {
        let payment = self.call_value().single_esdt();
        self.domain_token_id()
            .require_same_token(&payment.token_identifier);

        let attributes: DomainNftAttributes<Self::Api> = self
            .domain_token_id()
            .get_token_attributes(payment.token_nonce);

        require!(
            !self.is_expired(attributes.expiration),
            "Domain NFT expired"
        );

        let caller = self.blockchain().get_caller();

        self.domain_status(&attributes.domain).update(|status| {
            self.set_domain_manager(attributes.domain, domain_manager);
            *status = DomainStatus::Owned {
                owner: caller,
                nft_nonce: payment.token_nonce,
            }
        });
    }

    #[endpoint(nftizeDomain)]
    fn nftize_domain(&self, domain: ManagedBuffer) {
        let caller = self.blockchain().get_caller();

        let nft_nonce = self.domain_status(&domain).update(|status| {
            let (owner, nft_nonce) = if let DomainStatus::Owned { owner, nft_nonce } = status {
                (owner, *nft_nonce)
            } else {
                sc_panic!("Domain not owned");
            };

            require!(
                caller == *owner,
                "Only the domain's owner can NFTize the domain"
            );

            self.unset_domain_manager(domain);
            *status = DomainStatus::Nftized;

            nft_nonce
        });

        let token_identifier = self.domain_token_id().get_token_id();
        let amount = BigUint::from(1u64);
        self.send()
            .direct_esdt(&caller, &token_identifier, nft_nonce, &amount);
    }

    #[endpoint(resetDomain)]
    fn reset_domain(&self, domain: ManagedBuffer) {
        self.require_domain_not_in_blacklist(&domain);

        let expiration = self.domain_expiration(&domain).take();
        require!(self.is_expired(expiration), "Domain has not expired yet");
        let status = self.domain_status(&domain).take();
        require!(
            status != DomainStatus::ReservedByOwner,
            "Domain reserved by owner"
        );
    }

    #[only_owner]
    #[endpoint]
    fn reserve(&self, domain: ManagedBuffer, domain_manager: ManagedAddress) {
        self.domain_status(&domain)
            .set(DomainStatus::ReservedByOwner);

        self.set_domain_manager(domain, domain_manager);
    }

    fn set_domain_manager(&self, domain: ManagedBuffer, domain_manager: ManagedAddress) {
        let base_usernames_address = self.base_usernames_address().get();

        let () = self
            .base_usernames(base_usernames_address)
            .set_domain_manager(domain, domain_manager)
            .execute_on_dest_context();
    }

    fn unset_domain_manager(&self, domain: ManagedBuffer) {
        let base_usernames_address = self.base_usernames_address().get();

        let () = self
            .base_usernames(base_usernames_address)
            .unset_domain_manager(domain)
            .execute_on_dest_context();
    }

    fn create_domain_nft(&self, domain: ManagedBuffer, expiration: u64) -> EsdtTokenPayment {
        let token_id = self.domain_token_id().get_token_id();
        let amount = BigUint::from(1u32);
        let name = self.domain_token_name().get();
        let royalties = self.nft_royalties().get();
        let hash = ManagedBuffer::new();

        let attributes = DomainNftAttributes { domain, expiration };
        let uris: ManagedVec<ManagedBuffer> = ManagedVec::new();
        let token_nonce = self.send().esdt_nft_create(
            &token_id,
            &amount,
            &name,
            &royalties,
            &hash,
            &attributes,
            &uris,
        );
        EsdtTokenPayment::new(token_id, token_nonce, amount)
    }

    fn reserve_domain(&self, domain: &ManagedBuffer) -> u64 {
        self.require_domain_not_in_blacklist(domain);

        // TODO: domain name validation (allowed characters, max length etc.)

        self.domain_status(domain).update(|status| {
            require!(*status == DomainStatus::Available, "Domain not available");
            *status = DomainStatus::StartingAuction
        });

        let expiration =
            self.blockchain().get_block_timestamp() + self.domain_validity_period().get();

        self.domain_expiration(domain).set(expiration);
        expiration
    }

    fn burn_domain_nft(&self, domain_nft: EsdtTokenPayment) {
        let (token, nonce, amount) = domain_nft.into_tuple();
        self.send().esdt_local_burn(&token, nonce, &amount);
    }

    fn is_expired(&self, expiration: u64) -> bool {
        expiration <= self.blockchain().get_block_timestamp()
    }

    fn require_domain_not_in_blacklist(&self, domain: &ManagedBuffer) {
        require!(
            !self.domain_blacklist().contains(domain),
            "Domain is in blacklist"
        );
    }

    #[view(getBaseUsernamesAddress)]
    #[storage_mapper("baseUsernamesAddress")]
    fn base_usernames_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getDomainStatus)]
    #[storage_mapper("domainStatus")]
    fn domain_status(&self, domain: &ManagedBuffer) -> SingleValueMapper<DomainStatus<Self::Api>>;

    #[view(getDomainExpiration)]
    #[storage_mapper("domainExpiration")]
    fn domain_expiration(&self, domain: &ManagedBuffer) -> SingleValueMapper<u64>;

    #[view(getMarketplaceAddress)]
    #[storage_mapper("marketplaceAddress")]
    fn marketplace_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getNftRoyalties)]
    #[storage_mapper("nftRoyalties")]
    fn nft_royalties(&self) -> SingleValueMapper<BigUint>;

    #[view(getDomainValidityPeriod)]
    #[storage_mapper("domainValidityPeriod")]
    fn domain_validity_period(&self) -> SingleValueMapper<u64>;

    #[view(getAuctionMinBid)]
    #[storage_mapper("auctionMinBid")]
    fn auction_min_bid(&self) -> SingleValueMapper<BigUint>;

    #[view(getAuctionDuration)]
    #[storage_mapper("auctionDuration")]
    fn auction_duration(&self) -> SingleValueMapper<u64>;

    #[view(getDomainTokenId)]
    #[storage_mapper("domainTokenId")]
    fn domain_token_id(&self) -> NonFungibleTokenMapper;

    #[view(getDomainTokenName)]
    #[storage_mapper("domainTokenName")]
    fn domain_token_name(&self) -> SingleValueMapper<ManagedBuffer>;

    #[view(getDomainBlacklist)]
    #[storage_mapper("domainBlacklist")]
    fn domain_blacklist(&self) -> UnorderedSetMapper<ManagedBuffer>;

    #[proxy]
    fn esdt_nft_marketplace(
        &self,
        sc_address: ManagedAddress,
    ) -> esdt_nft_marketplace::Proxy<Self::Api>;

    #[proxy]
    fn base_usernames(&self, address: ManagedAddress) -> base_usernames::Proxy<Self::Api>;
}
