use flip::storage::StorageModule;
use flip::admin::AdminModule;
use elrond_wasm::types::{Address, EgldOrEsdtTokenIdentifier};
use elrond_wasm_debug::{rust_biguint, testing_framework::*, DebugApi, managed_biguint, managed_token_id_wrapped};
use elrond_wasm_debug::tx_mock::TxResult;
use flip::*;

pub type RustBigUint = num_bigint::BigUint;

const WASM_PATH: &'static str = "output/flip.wasm";
pub const EGLD_TOKEN_ID: &[u8] = b"EGLD";
const OWNER_BALANCE: u64 = 1100;// 1100 because 100 are useful for increase reserve = 100
const EGLD:u64 = 1000;
pub const FLIP_TOKEN_ID: &[u8] = b"FLIP-123456";
pub(crate) const TEN_PERCENT: u64 = 10_000_000;

pub struct FlipContractSetup<FlipContractObjBuilder>
    where
        FlipContractObjBuilder: 'static + Copy + Fn() -> flip::ContractObj<DebugApi>,
{
    pub blockchain_wrapper: BlockchainStateWrapper,
    pub owner_address: Address,
    pub alice: Address,
    pub bob: Address,
    pub carol: Address,
    pub contract_wrapper: ContractObjWrapper<flip::ContractObj<DebugApi>, FlipContractObjBuilder>,
}

impl <FlipContractObjBuilder> FlipContractSetup<FlipContractObjBuilder>
    where
        FlipContractObjBuilder: 'static + Copy + Fn() -> flip::ContractObj<DebugApi> {
    pub fn new(flip_builder: FlipContractObjBuilder) -> Self
    {
        let rust_zero = rust_biguint!(0u64);
        let mut blockchain_wrapper = BlockchainStateWrapper::new();
        let owner_address = blockchain_wrapper.create_user_account(&rust_biguint!(OWNER_BALANCE));
        let alice = blockchain_wrapper.create_user_account(&rust_biguint!(EGLD));
        let bob = blockchain_wrapper.create_user_account(&rust_biguint!(EGLD));
        let carol = blockchain_wrapper.create_user_account(&rust_biguint!(EGLD));
        let flip_wrapper = blockchain_wrapper.create_sc_account(
            &rust_zero,
            Some(&owner_address),
            flip_builder,
            WASM_PATH,
        );

        blockchain_wrapper.set_esdt_balance(&owner_address, &FLIP_TOKEN_ID, &rust_biguint!(OWNER_BALANCE));
        blockchain_wrapper.set_esdt_balance(&alice, &FLIP_TOKEN_ID, &rust_biguint!(1000));
        blockchain_wrapper.set_esdt_balance(&bob, &FLIP_TOKEN_ID, &rust_biguint!(1000));
        blockchain_wrapper.set_esdt_balance(&carol, &FLIP_TOKEN_ID, &rust_biguint!(1000));

        //Set init
        blockchain_wrapper
            .execute_tx(&owner_address, &flip_wrapper, &rust_zero, |sc| {
                sc.init(TEN_PERCENT, TEN_PERCENT, 10);
            })
            .assert_ok();

        //set maximum bet percent EGLD
        blockchain_wrapper
            .execute_tx(&owner_address, &flip_wrapper, &rust_biguint!(0), |sc| {
                sc.set_maximum_bet_percent(
                    EgldOrEsdtTokenIdentifier::egld(),
                    0,
                    TEN_PERCENT
                )
            }).assert_ok();

        //set maximum bet EGLD
        blockchain_wrapper
            .execute_tx(&owner_address, &flip_wrapper, &rust_biguint!(0), |sc| {
                sc.set_maximum_bet(
                    EgldOrEsdtTokenIdentifier::egld(),
                    0,
                    managed_biguint!(10)
                )
            }
            ).assert_ok();

        //set maximum bet percent ESDT
        blockchain_wrapper
            .execute_tx(&owner_address, &flip_wrapper, &rust_biguint!(0), |sc| {
                sc.set_maximum_bet_percent(
                    EgldOrEsdtTokenIdentifier::esdt(FLIP_TOKEN_ID),
                    0,
                    TEN_PERCENT
                )
            }).assert_ok();

        //set maximum bet ESDT
        blockchain_wrapper
            .execute_tx(&owner_address, &flip_wrapper, &rust_biguint!(0), |sc| {
                sc.set_maximum_bet(
                    EgldOrEsdtTokenIdentifier::esdt(FLIP_TOKEN_ID),
                    0,
                    managed_biguint!(10)
                )
            }
            ).assert_ok();

        // set minimum block bounty
        blockchain_wrapper
            .execute_tx(&owner_address, &flip_wrapper, &rust_biguint!(0), |sc| {
                sc.set_minimum_block_bounty(2u64)
            }
            ).assert_ok();


        blockchain_wrapper.add_mandos_set_account(flip_wrapper.address_ref());

        Self {
            blockchain_wrapper,
            owner_address,
            alice,
            bob,
            carol,
            contract_wrapper: flip_wrapper,
        }
    }
    pub fn increase_reserve(&mut self, token: &[u8], amount : u64) -> TxResult{
        if token == b"EGLD"{
            self
                .blockchain_wrapper
                .execute_tx(
                    &self.owner_address,
                    &self.contract_wrapper,
                    &rust_biguint!(amount),
                    |sc|{
                        sc.increase_reserve();
                    }
                )
        }else {
            self
                .blockchain_wrapper
                .execute_esdt_transfer(
                    &self.owner_address,
                    &self.contract_wrapper,
                    &FLIP_TOKEN_ID,
                    0,
                    &rust_biguint!(amount),
                    |sc|{
                        sc.increase_reserve();
                    }
                )
        }
    }

    pub fn execute_flip(&mut self,address:&Address, token : &[u8],amount:&RustBigUint)-> TxResult{
        if token == b"EGLD"{
            self
                .blockchain_wrapper
                .execute_tx(
                    address,
                    &self.contract_wrapper,
                    &amount,
                    |sc| {
                        sc.flip()
                    }
                )
        }else {
            self
            .blockchain_wrapper
            .execute_esdt_transfer(
                &address,
                &self.contract_wrapper,
                &FLIP_TOKEN_ID,
                0,
                &rust_biguint!(10),
                |sc| {
                    sc.flip()
                }
            )

        }
    }

    pub fn execute_flip_bounty(&mut self,address:&Address){
        self
            .blockchain_wrapper
            .execute_tx(
                address,
                &self.contract_wrapper,
                &rust_biguint!(0),
                |sc| {
                    sc.flip_bounty()

                }).assert_ok();
    }
}

#[test]
fn set_maximum_bet_percent_test(){
    let mut setup = FlipContractSetup::new(flip::contract_obj);

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc|{
                sc.set_maximum_bet_percent(
                    managed_token_id_wrapped!(FLIP_TOKEN_ID),
                    0,
                    TEN_PERCENT
                )
            }

        ).assert_ok();

    setup
        .blockchain_wrapper
        .execute_query(
            &setup.contract_wrapper,|sc|{
                let maximum_bet_percent = sc.maximum_bet_percent(&managed_token_id_wrapped!(FLIP_TOKEN_ID), 0).get();
                let expected = TEN_PERCENT;

                assert_eq!(maximum_bet_percent,expected)
            }
        ).assert_ok()
}

#[test]
fn set_maximum_bet_test(){
    let mut setup = FlipContractSetup::new(flip::contract_obj);

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc|{
                sc.set_maximum_bet(
                    managed_token_id_wrapped!(FLIP_TOKEN_ID),
                    0,
                    managed_biguint!(10)
                )
            }
        ).assert_ok();

    setup
        .blockchain_wrapper
        .execute_query(
            &setup.contract_wrapper,|sc|{
                let maximum_bet = sc.maximum_bet(&managed_token_id_wrapped!(FLIP_TOKEN_ID), 0).get();
                let expected = managed_biguint!(10);

                assert_eq!(maximum_bet,expected)
            }
        ).assert_ok()
}

#[test]
fn set_minimum_block_bounty(){
    let mut setup = FlipContractSetup::new(flip::contract_obj);

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc|{
                sc.set_minimum_block_bounty(2u64)
            }
        ).assert_ok();

    setup
        .blockchain_wrapper
        .execute_query(
            &setup.contract_wrapper,|sc|{
                let minimum_block_bounty = sc.minimum_block_bounty().get();
                let expected = 2u64;

                assert_eq!(minimum_block_bounty,expected)
            }
        ).assert_ok()

}


