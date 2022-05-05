use crate::storage;
elrond_wasm::imports!();

#[elrond_wasm::derive::module]
pub trait AdminModule:// ContractBase +
    storage::StorageModule
{

    #[payable("*")]
    #[endpoint(increaseReserve)]
    fn increase_reserve(
        &self,
        #[payment_token] payment_token: TokenIdentifier<Self::Api>,
        #[payment_nonce] payment_nonce: u64,
        #[payment_amount] payment_amount: BigUint<Self::Api>
    ) {

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
        token_identifier: TokenIdentifier<Self::Api>,
        token_nonce: u64,
        amount: BigUint<Self::Api>
    ) {
        let token_reserve = self.token_reserve(
            &token_identifier,
            token_nonce
        ).get();

        require!(
            amount <= token_reserve,
            "amount too high"
        );

        self.send()
            .direct(
                &self.blockchain().get_owner_address(),
                &token_identifier,
                token_nonce,
                &amount,
                &[]
            );
    }

    #[only_owner]
    #[endpoint(setMaximumBetPercent)]
    fn set_maximum_bet_percent(
        &self,
        token_identifier: TokenIdentifier<Self::Api>,
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
        token_identifier: TokenIdentifier<Self::Api>,
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

}