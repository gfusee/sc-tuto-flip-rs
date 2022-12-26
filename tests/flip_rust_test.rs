use flip::storage::StorageModule;
use flip::admin::AdminModule;
use elrond_wasm::types::Address;
use elrond_wasm_debug::{rust_biguint, testing_framework::*, DebugApi, managed_biguint, managed_token_id};
use flip::*;

pub type RustBigUint = num_bigint::BigUint;

const WASM_PATH: &'static str = "output/flip.wasm";
const OWNER_BALANCE: u64 = 10_000_000_000_000_000_000;

struct FlipContractSetup<FlipContractObjBuilder>
where
    FlipContractObjBuilder: 'static + Copy + Fn() -> flip::ContractObj<DebugApi>,
{
    pub blockchain_wrapper: BlockchainStateWrapper,
    pub owner_address: Address,
    pub alice: Address,
    pub bob: Address,
    pub contract_wrapper: ContractObjWrapper<flip::ContractObj<DebugApi>, FlipContractObjBuilder>,
}

fn setup_flip<FlipContractObjBuilder>(
    cf_builder: FlipContractObjBuilder,
) -> FlipContractSetup<FlipContractObjBuilder>
where
    FlipContractObjBuilder: 'static + Copy + Fn() -> flip::ContractObj<DebugApi>,
{
    let rust_zero = rust_biguint!(0u64);
    let mut blockchain_wrapper = BlockchainStateWrapper::new();
    let owner_address = blockchain_wrapper.create_user_account(&rust_biguint!(OWNER_BALANCE));
    let alice = blockchain_wrapper.create_user_account(&rust_zero);
    let bob = blockchain_wrapper.create_user_account(&rust_zero);
    let flip_wrapper = blockchain_wrapper.create_sc_account(
        &rust_zero,
        Some(&owner_address),
        cf_builder,
        WASM_PATH,
    );


    blockchain_wrapper
        .execute_tx(&owner_address, &flip_wrapper, &rust_zero, |sc| {
            sc.init(10, 10, 10);
        })
        .assert_ok();

    blockchain_wrapper.add_mandos_set_account(flip_wrapper.address_ref());

    FlipContractSetup {
        blockchain_wrapper,
        owner_address,
        alice,
        bob,
        contract_wrapper: flip_wrapper,
    }
}

#[test]
fn deploy_test() {
    let mut setup = setup_flip(flip::contract_obj);

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
fn increase_withdraw_reserve_test(){
    let mut setup = setup_flip(flip::contract_obj);

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(100),
            |sc|{
                sc.increase_reserve(
                    managed_token_id!(b"EGLD-123456"),
                    0,
                    managed_biguint!(100)
                );
            }
        ).assert_ok();

    setup
        .blockchain_wrapper
        .execute_query(&setup.contract_wrapper,|sc|{
            let actual_reserve= sc.token_reserve(&managed_token_id!(b"EGLD-123456"),0).get();
            let expected_reserve = managed_biguint!(100);

            assert_eq!(actual_reserve,expected_reserve);

            println!("{:?}", actual_reserve);
            println!("{:?}", expected_reserve)

        }).assert_ok();

    setup.blockchain_wrapper.check_egld_balance(setup.contract_wrapper.address_ref(),&rust_biguint!(100));


    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc|{
                sc.withdraw_reserve(
                    managed_token_id!(b"EGLD-123456"),
                    0,
                    managed_biguint!(100)

                )
            }
        ).assert_ok()

}


