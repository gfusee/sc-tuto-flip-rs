elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi)]
pub struct Flip<M : ManagedTypeApi> {
    pub id: u64,
    pub player_address: ManagedAddress<M>,
    pub token_identifier: EgldOrEsdtTokenIdentifier<M>,
    pub token_nonce: u64,
    pub amount: BigUint<M>,
    pub bounty: BigUint<M>,
    pub block_nonce: u64,
    pub minimum_block_bounty: u64
}