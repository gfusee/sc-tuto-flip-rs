use crate::storage;
elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::derive::module]
pub trait AdminModule:ContractBase +
    storage::StorageModule
{

    #[payable("*")]
    #[endpoint(increaseReserve)]
    fn increase_reserve(&self) {
        let (payment_token, payment_nonce, payment_amount) = self.call_value().egld_or_single_esdt().into_tuple();

        require!(
            payment_amount > 0u64,
            "zero payment"
        );

        self.token_reserve(
            &payment_token,
            payment_nonce
        ).update(|reserve| *reserve += payment_amount);

    }

    #[only_owner]
    #[endpoint(withdrawReserve)]
    fn withdraw_reserve(
        &self,
        token_identifier: EgldOrEsdtTokenIdentifier<Self::Api>,
        token_nonce: u64,
        amount: OptionalValue<BigUint<Self::Api>>
    ) {
        let reserve_mapper = self.token_reserve(&token_identifier, token_nonce).get();
        let withdraw_amount = match amount {
            OptionalValue::Some(amt) => amt,
            OptionalValue::None => reserve_mapper
        };

        self.token_reserve(
            &token_identifier,
            token_nonce
        ).update(|reserve|{
            require!(withdraw_amount > 0 && withdraw_amount <= *reserve,
            "Invalid withdraw amount"
            );

            *reserve -= &withdraw_amount;
        });

        self.send()
            .direct(
                &self.blockchain().get_caller(),
                &token_identifier,
                0,
                &withdraw_amount
            );
    }

    #[only_owner]
    #[endpoint(setMaximumBetPercent)]
    fn set_maximum_bet_percent(
        &self,
        token_identifier: EgldOrEsdtTokenIdentifier<Self::Api>,
        token_nonce: u64,
        percent: u64
    ) {

        require!(
            percent > 0u64,
            "percent zero"
        );

        self.maximum_bet_percent(
            &token_identifier,
            token_nonce
        ).set(percent);

    }

    #[only_owner]
    #[endpoint(setMaximumBet)]
    fn set_maximum_bet(
        &self,
        token_identifier: EgldOrEsdtTokenIdentifier<Self::Api>,
        token_nonce: u64,
        amount: BigUint<Self::Api>
    ) {

        require!(
            amount > 0u64,
            "amount zero"
        );

        self.maximum_bet(
            &token_identifier,
            token_nonce
        ).set(amount);

    }

    #[only_owner]
    #[endpoint(setMinimumBlockBounty)]
    fn set_minimum_block_bounty(
        &self,
        minimum_block_bounty: u64
    ) {

        require!(
            minimum_block_bounty > 0u64,
            "minimum_block_bounty zero"
        );

        self.minimum_block_bounty().set(minimum_block_bounty);

    }

}