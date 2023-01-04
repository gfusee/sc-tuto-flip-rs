use elrond_wasm::elrond_codec::multi_types::OptionalValue;
use flip::storage::StorageModule;
use flip::admin::AdminModule;
use elrond_wasm_debug::{rust_biguint, testing_framework::*, managed_biguint, managed_egld_token_id, managed_token_id_wrapped, managed_address};

use flip::*;

pub type RustBigUint = num_bigint::BigUint;

mod flip_setup;
use flip_setup::*;
use crate::flip_setup::{EGLD_TOKEN_ID,FLIP_TOKEN_ID};


#[test]
fn deploy_test() {
    let mut setup = FlipContractSetup::new(flip::contract_obj);

    // simulate deploy
    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.init(10, 10, 10);
            },
        )
        .assert_ok();

}

#[test]
fn increase_reserve() {
    let mut setup = FlipContractSetup::new(flip::contract_obj);
    let amount:u64 = 200;

    //Increase EGLD reserve
    setup.increase_reserve(&EGLD_TOKEN_ID, amount)
        .assert_ok();

    //Check ESDT reserve
    setup
        .blockchain_wrapper
        .execute_query(&setup.contract_wrapper,|sc|{
            let actual_reserve= sc.token_reserve(&managed_egld_token_id!(),0).get();
            let expected_reserve = managed_biguint!(200);

            assert_eq!(actual_reserve,expected_reserve);

        }).assert_ok();

    //Increase ESDT reserve
    setup.increase_reserve(&FLIP_TOKEN_ID, amount)
        .assert_ok();

    //Check ESDT reserve
    setup
        .blockchain_wrapper
        .execute_query(&setup.contract_wrapper,|sc|{
            let actual_reserve= sc.token_reserve(&managed_token_id_wrapped!(FLIP_TOKEN_ID), 0).get();
            let expected_reserve = managed_biguint!(200);

            assert_eq!(actual_reserve,expected_reserve);

        }).assert_ok();

    setup.blockchain_wrapper.check_egld_balance(&setup.contract_wrapper.address_ref(),&rust_biguint!(200));
    setup.blockchain_wrapper.check_esdt_balance(&setup.contract_wrapper.address_ref(),FLIP_TOKEN_ID,&rust_biguint!(200));

}

#[test]
fn withdraw_reserve() {

    let mut setup = FlipContractSetup::new(flip::contract_obj);

    setup.increase_reserve(&EGLD_TOKEN_ID,200).assert_ok();
    setup.increase_reserve(&FLIP_TOKEN_ID,200).assert_ok();

    //Withdraw EGLD
    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc|{
                sc.withdraw_reserve(
                    managed_egld_token_id!(),
                    0,
                    OptionalValue::Some(managed_biguint!(100))

                )
            }
        ).assert_ok();

    //check reserve and balance EGLD
    setup
        .blockchain_wrapper
        .execute_query(&setup.contract_wrapper,|sc|{
            let actual_reserve= sc.token_reserve(&managed_egld_token_id!(),0).get();
            let expected_reserve = managed_biguint!(100);

            assert_eq!(actual_reserve,expected_reserve);

            println!("{:?}", actual_reserve);
            println!("{:?}", expected_reserve)

        }).assert_ok();

    setup.blockchain_wrapper.check_egld_balance(setup.contract_wrapper.address_ref(),&rust_biguint!(100));
    setup.blockchain_wrapper.check_esdt_balance(setup.contract_wrapper.address_ref(), FLIP_TOKEN_ID, &rust_biguint!(200));

    //Withdraw ESDT
    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc|{
                sc.withdraw_reserve(
                    managed_token_id_wrapped!(FLIP_TOKEN_ID),
                    0,
                    OptionalValue::Some(managed_biguint!(100))

                )
            }
        ).assert_ok();

    //check reserve and balance
    setup
        .blockchain_wrapper
        .execute_query(&setup.contract_wrapper,|sc|{
            let actual_reserve= sc.token_reserve(&managed_token_id_wrapped!(FLIP_TOKEN_ID), 0).get();
            let expected_reserve = managed_biguint!(100);

            assert_eq!(actual_reserve,expected_reserve);

            println!("{:?}", actual_reserve);
            println!("{:?}", expected_reserve)

        }).assert_ok();

    setup.blockchain_wrapper.check_egld_balance(setup.contract_wrapper.address_ref(),&rust_biguint!(100));
    setup.blockchain_wrapper.check_esdt_balance(setup.contract_wrapper.address_ref(), FLIP_TOKEN_ID, &rust_biguint!(100));


}

#[test]
fn single_flip_egld() {
    let mut setup = FlipContractSetup::new(flip::contract_obj);
    let amount = rust_biguint!(10);
    let alice = setup.alice.clone();

    setup.increase_reserve(&EGLD_TOKEN_ID,100).assert_ok();
    setup.blockchain_wrapper.set_block_nonce(13);
    setup.execute_flip(&alice,&EGLD_TOKEN_ID,&amount).assert_ok();

    setup
        .blockchain_wrapper
        .execute_query(
            &setup.contract_wrapper,
            |sc| {
                let struct_flip = sc.flip_for_id(1).get();
                let address = struct_flip.player_address;
                assert_eq!(address, managed_address!(&alice));

                let token_id = struct_flip.token_identifier;
                let token_expected = managed_egld_token_id!();
                assert_eq!(token_id, token_expected);

                let token_nonce = struct_flip.token_nonce;
                let nonce_expected = 0 as u64;
                assert_eq!(token_nonce, nonce_expected);

                let amount = struct_flip.amount;
                let expected_amount = managed_biguint!(8);
                assert_eq!(amount, expected_amount);

                let bounty = struct_flip.bounty;
                let expected_bounty = managed_biguint!(1);
                assert_eq!(bounty, expected_bounty);

                let block_nonce = struct_flip.block_nonce;
                let expected_nonce = 13;
                assert_eq!(block_nonce,expected_nonce);

                let minimum_block_bounty = struct_flip.minimum_block_bounty;
                let expected_block_bounty = 2;
                assert_eq!(minimum_block_bounty,expected_block_bounty);
            }
        ).assert_ok();
}

#[test]
fn single_flip_esdt() {
    let mut setup = FlipContractSetup::new(flip::contract_obj);
    let amount = rust_biguint!(10);
    let alice = setup.alice.clone();

    setup.increase_reserve(&FLIP_TOKEN_ID,100).assert_ok();
    setup.blockchain_wrapper.set_block_nonce(13);
    setup.execute_flip(&alice,&FLIP_TOKEN_ID,&amount).assert_ok();

    setup
        .blockchain_wrapper
        .execute_query(
            &setup.contract_wrapper,
            |sc| {
                let struct_flip = sc.flip_for_id(1).get();
                let address = struct_flip.player_address;
                assert_eq!(address, managed_address!(&alice));

                let token_id = struct_flip.token_identifier;
                let token_expected = managed_token_id_wrapped!(FLIP_TOKEN_ID);
                assert_eq!(token_id, token_expected);

                let token_nonce = struct_flip.token_nonce;
                let nonce_expected = 0 as u64;
                assert_eq!(token_nonce, nonce_expected);

                let amount = struct_flip.amount;
                let expected_amount = managed_biguint!(8);
                assert_eq!(amount, expected_amount);


                let bounty = struct_flip.bounty;
                let expected_bounty = managed_biguint!(1);
                assert_eq!(bounty, expected_bounty);


                let block_nonce = struct_flip.block_nonce;
                let expected_nonce = 13;
                assert_eq!(block_nonce,expected_nonce);

                let minimum_block_bounty = struct_flip.minimum_block_bounty;
                let expected_block_bounty = 2;
                assert_eq!(minimum_block_bounty,expected_block_bounty);
            }
        ).assert_ok();

}

#[test]
fn single_flip_win_lose_egld() {
    let mut setup = FlipContractSetup::new(flip::contract_obj);
    let amount = rust_biguint!(10);
    let alice = setup.alice.clone();
    let carol = setup.carol.clone();
    let owner = setup.owner_address.clone();

    setup.increase_reserve(&EGLD_TOKEN_ID,100).assert_ok();

    setup.execute_flip(&alice,&EGLD_TOKEN_ID,&amount).assert_ok();
    setup.execute_flip(&alice,&EGLD_TOKEN_ID,&amount).assert_ok();

    setup.blockchain_wrapper.set_block_nonce(2);
    setup.execute_flip_bounty(&carol);

    setup.blockchain_wrapper.check_egld_balance(&carol,&rust_biguint!(1002));
    setup.blockchain_wrapper.check_egld_balance(&alice,&rust_biguint!(996));
    setup.blockchain_wrapper.check_egld_balance(&owner,&rust_biguint!(1002));

}

#[test]
fn single_flip_win_lose_esdt() {
    let mut setup = FlipContractSetup::new(flip::contract_obj);
    let amount =rust_biguint!(10);
    let alice = setup.alice.clone();
    let carol = setup.carol.clone();
    let owner = setup.owner_address.clone();

    setup.increase_reserve(&FLIP_TOKEN_ID,100).assert_ok();

    setup.execute_flip(&alice,&FLIP_TOKEN_ID,&amount).assert_ok();
    setup.execute_flip(&alice,&FLIP_TOKEN_ID,&amount).assert_ok();

    setup.blockchain_wrapper.set_block_nonce(2);
    setup.execute_flip_bounty(&carol);

    setup.blockchain_wrapper.check_esdt_balance(&carol,&FLIP_TOKEN_ID,&rust_biguint!(1002));
    setup.blockchain_wrapper.check_esdt_balance(&alice,&FLIP_TOKEN_ID,&rust_biguint!(996));
    setup.blockchain_wrapper.check_esdt_balance(&owner,&FLIP_TOKEN_ID,&rust_biguint!(1002));
}

#[test]
fn multiple_flip_bounty() {
    let mut setup = FlipContractSetup::new(flip::contract_obj);
    let amount = rust_biguint!(10);
    let alice = setup.alice.clone();
    let bob = setup.bob.clone();
    let carol = setup.carol.clone();

    setup.increase_reserve(&EGLD_TOKEN_ID,100).assert_ok();
    setup.increase_reserve(&FLIP_TOKEN_ID,100).assert_ok();

    setup.execute_flip(&alice,&EGLD_TOKEN_ID,&amount).assert_ok();
    setup.execute_flip(&bob,&EGLD_TOKEN_ID,&amount).assert_ok();

    // set new block nonce
    setup.blockchain_wrapper.set_block_nonce(2);
    setup.execute_flip(&alice,&FLIP_TOKEN_ID,&amount).assert_ok();

    setup.blockchain_wrapper.set_block_nonce(11);
    setup.execute_flip(&alice,&FLIP_TOKEN_ID,&amount).assert_ok();
    setup.execute_flip(&bob,&FLIP_TOKEN_ID,&amount).assert_ok();

    setup.execute_flip_bounty(&carol)
}