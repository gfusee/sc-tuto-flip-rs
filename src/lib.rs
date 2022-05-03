#![no_std]

use core::cmp::min;
use crate::structs::Flip;

const HUNDRED_PERCENT: u64 = 10000;

mod storage;
mod structs;elrond_wasm::imports!();

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[elrond_wasm::derive::contract]
pub trait FlipContract:// ContractBase +
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

        let max_allowed_bet = min(
            self.maximum_profit().get(),
            token_reserve * self.maximum_profit_percent().get() / HUNDRED_PERCENT
        );

        require!(
            payment_amount <= max_allowed_bet,
            "bet too high"
        );

        let owner_profits = &payment_amount * self.owner_percent_fees().get() / HUNDRED_PERCENT;
        let bounty = &payment_amount * self.bounty_percent().get() / HUNDRED_PERCENT;
        let amount = &payment_amount - &bounty - &owner_profits;

        require!(
            max_allowed_profit >= max_player_profit,
            "too much bet"
        );

        let flip = Flip {
            id: flip_id,
            player_address: self.blockchain().get_caller(),
            token_identifier: payment_token.clone(),
            token_nonce: payment_nonce,
            amount,
            bounty,
            block_nonce: self.blockchain().get_block_nonce(),
            minimum_block_bounty: self.minimum_block_bounty().get()
        };

        self.send()
            .direct(
                &self.blockchain().get_owner_address(),
                &payment_token,
                payment_nonce,
                &owner_profits,
                &[]
            );

        let reserved_tokens = &amount * BigUint::from(2u8) + &bounty;

        self.token_reserve(
            &payment_token,
            payment_nonce
        ).update(|reserve| *reserve -= &reserved_tokens);

        self.flip_for_id(flip_id).set(flip);
        self.last_flip_id().set(flip_id);

    }

    #[endpoint(flipBounty)]
    fn flip_bounty(
        &self
    ) {
        let caller = self.blockchain().get_caller();

        require!(
            !self.blockchain().is_smart_contract(&caller),
            "caller is a smart contract"
        );

        let last_bounty_flip_id = self.last_bounty_flip_id().get();
        let last_flip_id = self.last_flip_id().get();

        require!(
            last_bounty_flip_id < last_flip_id,
            "last bounty flip id >= last flip id"
        );

        let current_block_nonce = self.blockchain().get_block_nonce();

        let mut bounty_flip_id = last_bounty_flip_id;

        while bounty_flip_id < last_flip_id {
            let flip_id = bounty_flip_id + 1u64;

            if self.flip_for_id(flip_id).is_empty() {
                break;
            }

            let flip = self.flip_for_id(flip_id).get();

            if current_block_nonce <= flip.block_nonce + flip.minimum_block_bounty {
                break;
            }

            self.make_flip(
                &caller,
                &flip
            );

            bounty_flip_id += 1u64;
        }

        self.last_bounty_flip_id().set(bounty_flip_id);

    }

    fn make_flip(
        &self,
        bounty_address: &ManagedAddress<Self::Api>,
        flip: &Flip<Self::Api>
    ) {

        let mut rand_source = RandomnessSource::<Self::Api>::new();
        let random_number = rand_source.next_u8_in_range(0, 2);
        let is_win = random_number == 1u8;

        self.send()
            .direct(
                &bounty_address,
                &flip.token_identifier,
                flip.token_nonce,
                &flip.bounty,
                &[]
            );

        let profit_if_win = &flip.amount * BigUint::from(2u8);

        if is_win {
            self.send()
                .direct(
                    &self.blockchain().get_owner_address(),
                    &flip.token_identifier,
                    flip.token_nonce,
                    &profit_if_win,
                    &[]
                );
        } else {
            self.token_reserve(
                &flip.token_identifier,
                flip.token_nonce
            )
                .update(|reserve| *reserve += &profit_if_win);
        }

        self.flip_for_id(flip.id).clear();


    }

}
