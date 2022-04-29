use crate::structs::Flip;
elrond_wasm::imports!();

#[elrond_wasm::derive::module]
pub trait StorageModule {

    #[view(ownerPercentFees)]
    #[storage_mapper("owner_percent_fees")]
    fn owner_percent_fees(&self) -> SingleValueMapper<Self::Api, u64>;

    #[view(incentivePercentFees)]
    #[storage_mapper("incentive_percent_fees")]
    fn incentive_percent_fees(&self) -> SingleValueMapper<Self::Api, u64>;

    #[storage_mapper("maximum_bet_percent")]
    fn maximum_bet_percent(&self) -> SingleValueMapper<Self::Api, u64>;

    #[storage_mapper("minimum_blocks_bounty")]
    fn minimum_blocks_bounty(&self) -> SingleValueMapper<Self::Api, u64>;

    #[view(getTokenReserve)]
    #[storage_mapper("token_reserve")]
    fn token_reserve(
        &self,
        token_identifier: &TokenIdentifier<Self::Api>,
        token_nonce: u64
    ) -> SingleValueMapper<Self::Api, BigUint<Self::Api>>;

    #[view(flipForId)]
    #[storage_mapper("flip_for_id")]
    fn flip_for_id(&self, id: u64) -> SingleValueMapper<Self::Api, Flip<Self::Api>>;

    #[view(bountyAmount)]
    #[storage_mapper("bounty_amount")]
    fn bounty_amount(&self) -> SingleValueMapper<Self::Api, BigInt<Self::Api>>;

    #[view(lastBountyFlipId)]
    #[storage_mapper("last_bounty_flip_id")]
    fn last_bounty_flip_id(&self) -> SingleValueMapper<Self::Api, u64>;

    #[view(lastFlipId)]
    #[storage_mapper("last_flip_id")]
    fn last_flip_id(&self) -> SingleValueMapper<Self::Api, u64>;

}