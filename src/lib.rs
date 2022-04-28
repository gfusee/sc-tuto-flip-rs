#![no_std]

const HUNDRED_PERCENT: u64 = 10000;

mod storage;elrond_wasm::imports!();

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[elrond_wasm::derive::contract]
pub trait FlipContract:
    storage::StorageModule
{

    #[init]
    fn init(
        &self,
        owner_percent_fees: u64,
        incentive_percent_fees: u64
    ) {
        self.owner_percent_fees().set(owner_percent_fees);
        self.incentive_percent_fees().set(incentive_percent_fees);
    }

    #[payable("*")]
    #[endpoint]
    fn flip(
        &self,
        #[payment_amount] payment_amount: BigInt<Self::Api>,
        #[payment_token] payment_token: TokenIdentifier<Self::Api>
    ) {

    }

    #[endpoint]
    fn flip_bounty(
        &self
    )

}
