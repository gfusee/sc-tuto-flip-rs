elrond_wasm::imports!();

#[elrond_wasm::derive::module]
pub trait StorageModule {

    #[storage_mapper("owner_percent_fees")]
    fn owner_percent_fees(&self) -> SingleValueMapper<Self::Api, u64>;

    #[storage_mapper("incentive_percent_fees")]
    fn incentive_percent_fees(&self) -> SingleValueMapper<Self::Api, u64>;

    #[storage_mapper("last_bounty_flip_id")]
    fn last_bounty_flip_id(&self) -> SingleValueMapper<Self::Api, u64>;

    #[storage_mapper("bounty_flip_id")]
    fn bounty_flip_id(&self) -> SingleValueMapper<Self::Api, u64>;

}