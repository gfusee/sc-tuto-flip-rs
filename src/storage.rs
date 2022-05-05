use crate::structs::Flip;
elrond_wasm::imports!();

#[elrond_wasm::derive::module]
pub trait StorageModule {

    #[view(getOwnerPercentFees)]
    #[storage_mapper("owner_percent_fees")]
    fn owner_percent_fees(&self) -> SingleValueMapper<Self::Api, u64>;

    #[view(getMaximumBet)]
    #[storage_mapper("maximum_bet")]
    fn maximum_bet(
        &self,
        token_identifier: &TokenIdentifier<Self::Api>,
        token_nonce: u64
    ) -> SingleValueMapper<Self::Api, BigUint<Self::Api>>;

    #[view(getMaximumProfitPercent)]
    #[storage_mapper("maximum_profit_percent")]
    fn maximum_bet_percent(
        &self,
        token_identifier: &TokenIdentifier<Self::Api>,
        token_nonce: u64
    ) -> SingleValueMapper<Self::Api, u64>;

    #[view(getMinimumBlockBounty)]
    #[storage_mapper("minimum_block_bounty")]
    fn minimum_block_bounty(&self) -> SingleValueMapper<Self::Api, u64>;

    #[view(getTokenReserve)]
    #[storage_mapper("token_reserve")]
    fn token_reserve(
        &self,
        token_identifier: &TokenIdentifier<Self::Api>,
        token_nonce: u64
    ) -> SingleValueMapper<Self::Api, BigUint<Self::Api>>;

    #[view(getBountyReserve)]
    #[storage_mapper("bounty_reserve")]
    fn bounty_reserve(
        &self,
        token_identifier: &TokenIdentifier<Self::Api>,
        token_nonce: u64
    ) -> SingleValueMapper<Self::Api, BigUint<Self::Api>>;

    #[view(flipForId)]
    #[storage_mapper("flip_for_id")]
    fn flip_for_id(&self, id: u64) -> SingleValueMapper<Self::Api, Flip<Self::Api>>;

    #[view(getBountyAmount)]
    #[storage_mapper("bounty_percent_fees")]
    fn bounty_percent_fees(&self) -> SingleValueMapper<Self::Api, u64>;

    #[view(getLastBountyFlipId)]
    #[storage_mapper("last_bounty_flip_id")]
    fn last_bounty_flip_id(&self) -> SingleValueMapper<Self::Api, u64>;

    #[view(getLastFlipId)]
    #[storage_mapper("last_flip_id")]
    fn last_flip_id(&self) -> SingleValueMapper<Self::Api, u64>;

}