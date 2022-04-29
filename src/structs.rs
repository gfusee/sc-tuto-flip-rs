elrond_wasm::imports!();

pub struct Flip<M : ManagedApi<Self::Api>> {
    pub id: u64,
    pub token_identifier: TokenIdentifier<M>,
    pub token_nonce: u64,
    pub amount: BigUInt<M>,
    pub block_nonce: u64
}