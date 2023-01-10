#![no_std]

use core::cmp::min;
use crate::structs::Flip;

const HUNDRED_PERCENT: u64 = 100_000_000;

pub mod storage;
pub mod structs;
pub mod admin;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::derive::contract]
pub trait FlipContract: ContractBase +
    storage::StorageModule + admin::AdminModule
{

    #[init]
    fn init(
        &self,
        owner_percent_fees: u64,
        bounty_percent_fees: u64,
        minimum_block_bounty: u64
    ) {
        self.owner_percent_fees().set(owner_percent_fees);
        self.bounty_percent_fees().set(bounty_percent_fees);

        require!(
            minimum_block_bounty > 0u64,
            "minimum_block_bounty is zero"
        );

        self.minimum_block_bounty().set(minimum_block_bounty)
    }

    #[payable("*")]
    #[endpoint]
    fn flip(
        &self) {
        let (token_id, nonce, payment_amount ) = self.call_value().egld_or_single_esdt().into_tuple();

        let token_reserve = self.token_reserve(
            &token_id,
            nonce
        ).get();

        require!(
            token_reserve > 0u64,
            "no token reserve"
        );

        require!(
            !self.maximum_bet(&token_id, nonce).is_empty(),
            "no maximum bet"
        );

        require!(
            !self.maximum_bet_percent(&token_id, nonce).is_empty(),
            "no maximum bet percent"
        );

        let maximum_bet = self.maximum_bet(
            &token_id,
            nonce
        ).get();

        let maximum_bet_percent = self.maximum_bet_percent(
            &token_id,
            nonce
        ).get();

        let max_allowed_bet = min(
            maximum_bet,
            token_reserve * &BigUint::from(maximum_bet_percent) / HUNDRED_PERCENT
        );

        let owner_profits = &payment_amount * &BigUint::from(self.owner_percent_fees().get()) / HUNDRED_PERCENT;
        let bounty = &payment_amount * &BigUint::from(self.bounty_percent_fees().get()) / HUNDRED_PERCENT;
        let amount = &payment_amount - &bounty - &owner_profits;

        require!(
            amount <= max_allowed_bet,
            "too much bet"
        );

        let last_flip_id = if self.last_flip_id().is_empty() {
            0u64
        } else {
            self.last_flip_id().get()
        };

        let flip_id = last_flip_id + 1;

        let flip = Flip {
            id: flip_id,
            player_address: self.blockchain().get_caller(),
            token_identifier: token_id.clone(),
            token_nonce: nonce,
            amount: amount.clone(),
            bounty: bounty.clone(),
            block_nonce: self.blockchain().get_block_nonce(),
            minimum_block_bounty: self.minimum_block_bounty().get()
        };

        self.token_reserve(
            &token_id,
            nonce
        ).update(|reserve| *reserve -= &amount);

        self.send().direct(
                &self.blockchain().get_owner_address(),
                &token_id,
                0,
                &owner_profits
        );

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

            if current_block_nonce < flip.block_nonce + flip.minimum_block_bounty {
                break;
            }

            self.make_flip(
                &caller,
                &flip
            );

            bounty_flip_id += 1u64;
        }

        if bounty_flip_id == last_bounty_flip_id {
            sc_panic!("no bounty")
        }

        self.last_bounty_flip_id().set(bounty_flip_id);

    }

    fn make_flip(
        &self,
        bounty_address: &ManagedAddress<Self::Api>,
        flip: &Flip<Self::Api>
    ) {

        let mut rand_source = RandomnessSource::new();
        let random_number = rand_source.next_u8_in_range(0, 2);
        let is_win = random_number == 1u8;

        self.send()
            .direct(
                &bounty_address,
                &flip.token_identifier,
                flip.token_nonce,
                &flip.bounty
            );

        let profit_if_win = &flip.amount * &BigUint::from(2u8);

        if is_win {
            self.send()
                .direct(
                    &flip.player_address,
                    &flip.token_identifier,
                    flip.token_nonce,
                    &profit_if_win
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
