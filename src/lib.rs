#![no_std]

use crate::structs::Flip;

const HUNDRED_PERCENT: u64 = 10000;

mod storage;
mod structs;elrond_wasm::imports!();

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[elrond_wasm::derive::contract]
pub trait FlipContract: ContractBase +
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
        #[payment_amount] payment_amount: BigUint<Self::Api>,
        #[payment_token] payment_token: TokenIdentifier<Self::Api>,
        #[payment_nonce] payment_nonce: u64
    ) {

        let last_flip_id = if self.last_flip_id().is_empty() {
            0u64
        } else {
            self.last_flip_id().get()
        };

        let flip_id = last_flip_id + 1;

        require!(
            !self.token_reserve(
                &payment_token,
                payment_nonce
            ).is_empty(),
            "no token reserve"
        );

        let token_reserve = self.token_reserve(
            &payment_token,
            payment_nonce
        ).get();

        require!(
            token_reserve > 0u64,
            "no reserve"
        );

        let max_player_profit = &payment_amount * &BigUint::from(2u64);
        let max_allowed_profit = token_reserve * self.maximum_bet_percent().get() / HUNDRED_PERCENT;

        require!(
            max_allowed_profit >= max_player_profit,
            "too much bet"
        );

        let flip = Flip {
            id: flip_id,
            player_address: self.blockchain().get_caller(),
            token_identifier: payment_token,
            token_nonce: payment_nonce,
            amount: payment_amount,
            block_nonce: self.blockchain().get_block_nonce()
        };

        self.token_reserve().update(|reserve| *reserve -= max_allowed_profit);
        self.flip_for_id(flip_id).set(flip);

    }

    #[endpoint]
    fn flip_bounty(
        &self
    ) {

    }

}
