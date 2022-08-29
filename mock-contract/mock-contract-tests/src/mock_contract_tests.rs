use crate::mock_contract_instance::MOCKCONTRACTInstance;
use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U256};
use casperlabs_test_env::{TestContract, TestEnv};
use common::keys::*;
fn deploy() -> (TestEnv, AccountHash, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let instance = MOCKCONTRACTInstance::new_deploy(&env, "MOCKCONTRACT", owner, "sa".to_string());
    (env, owner, instance)
}

#[test]
fn test_deploy() {
    let (_, _, _) = deploy();
}
#[test]
fn test_mint() {
    let (env, owner, instance) = deploy();
    let instance = MOCKCONTRACTInstance::contract_instance(instance);
    let to: Key = Key::Account(owner);
    let id: U256 = 1.into();
    let amount: U256 = 100.into();
    let data: String = "0x00".to_string();
    instance.mint(owner, to, id, amount, data);
    let arg_token_id: U256 = 1.into();
    let arg_owner: Key = Key::Account(owner);
    TestContract::new(
        &env,
        "erc1155-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF),
            "package_hash" => Key::Hash(instance.package_hash()),
            "owner"=>Key::from(owner),
            "account"=>arg_owner,
            "id"=>arg_token_id
        },
        0,
    );
    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, 100.into());
}
#[test]
fn test_balance_of() {
    let (env, owner, instance) = deploy();
    let instance = MOCKCONTRACTInstance::contract_instance(instance);
    let to: Key = Key::Account(owner);
    let id: U256 = 2.into();
    let amount: U256 = 50.into();
    let data: String = "0x00".to_string();
    instance.mint(owner, to, id, amount, data);
    let id: U256 = 2.into();
    let account: Key = Key::Account(owner);
    TestContract::new(
        &env,
        "erc1155-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF),
            "package_hash" => Key::Hash(instance.package_hash()),
            "owner"=>Key::from(owner),
            "account"=>account,
            "id"=>id
        },
        0,
    );
    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, 50.into());
}
#[test]
fn test_balance_of_batch() {
    let (env, owner, instance) = deploy();
    let instance = MOCKCONTRACTInstance::contract_instance(instance);
    let to1: Key = Key::Account(owner);
    let id1: U256 = 2.into();
    let amount1: U256 = 50.into();
    let data1: String = "0x00".to_string();
    instance.mint(owner, to1, id1, amount1, data1);
    let to2: Key = Key::Account(env.next_user());
    let id2: U256 = 2.into();
    let amount2: U256 = 100.into();
    let data2: String = "0x00".to_string();
    instance.mint(owner, to2, id2, amount2, data2);
    let accounts: Vec<String> = vec![to1.to_formatted_string(), to2.to_formatted_string()];
    let ids: Vec<U256> = vec![id1, id2];
    TestContract::new(
        &env,
        "erc1155-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF_BATCH),
            "package_hash" => Key::Hash(instance.package_hash()),
            "owner"=>Key::from(owner),
            "accounts"=>accounts,
            "ids"=>ids
        },
        0,
    );
    let ret: Vec<U256> = env.query_account_named_key(owner, &[BALANCE_OF_BATCH.into()]);
    assert_eq!(ret[0], 50.into());
    assert_eq!(ret[1], 100.into());
}
#[test]
fn test_is_approved_for_all() {
    let (env, owner, instance) = deploy();
    let account: Key = Key::Account(owner);
    let operator: Key = Key::Account(env.next_user());
    TestContract::new(
        &env,
        "erc1155-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(IS_APPROVED_FOR_ALL),
            "package_hash" => Key::Hash(instance.package_hash()),
            "owner"=>Key::from(owner),
            "account"=>account,
            "operator"=>operator
        },
        0,
    );
    let ret: bool = env.query_account_named_key(owner, &[IS_APPROVED_FOR_ALL.into()]);
    assert_eq!(ret,false);
}
#[test]
fn test_set_approval_for_all() {
    let (env, owner, instance) = deploy();
    let instance = MOCKCONTRACTInstance::contract_instance(instance);
    let operator: Key = Key::Account(env.next_user());
    let approved: bool = true;
    instance.set_approval_for_all(owner, operator, approved);
    let account: Key = Key::Account(owner);
    TestContract::new(
        &env,
        "erc1155-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(IS_APPROVED_FOR_ALL),
            "package_hash" => Key::Hash(instance.package_hash()),
            "owner"=>Key::from(owner),
            "account"=>account,
            "operator"=>operator
        },
        0,
    );
    let ret: bool = env.query_account_named_key(owner, &[IS_APPROVED_FOR_ALL.into()]);
    assert_eq!(ret,true);
}
#[test]
fn test_safe_transfer_from() {
    let (env, owner, instance) = deploy();
    let instance = MOCKCONTRACTInstance::contract_instance(instance);
    let to = env.next_user();
    let id: U256 = 1.into();
    let amount: U256 = 100.into();
    let data: String = "0x00".to_string();
    instance.mint(owner, Key::Account(to), id, amount, data);
    let approved: bool = true;
    instance.set_approval_for_all(owner, Key::Account(to), approved);
    let from = to;
    let transfer_to = Key::Account(owner);
    let data: String = "0x00".to_string();

    instance.safe_transfer_from(from, Key::Account(from), transfer_to, id, 40.into(), data);
    TestContract::new(
        &env,
        "erc1155-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF),
            "package_hash" => Key::Hash(instance.package_hash()),
            "owner"=>Key::from(owner),
            "account"=>Key::Account(from),
            "id"=>id
        },
        0,
    );
    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, 60.into());
    TestContract::new(
        &env,
        "erc1155-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF),
            "package_hash" => Key::Hash(instance.package_hash()),
            "owner"=>Key::from(owner),
            "account"=>transfer_to,
            "id"=>id
        },
        0,
    );
    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, 40.into());
}
#[test]
fn test_safe_batch_transfer_from() {
    let (env, owner, instance) = deploy();
    let instance = MOCKCONTRACTInstance::contract_instance(instance);
    let to = env.next_user();
    let id1: U256 = 1.into();
    let amount1: U256 = 100.into();
    let data1: String = "0x00".to_string();
    instance.mint(owner, Key::Account(to), id1, amount1, data1);
    let id2: U256 = 2.into();
    let amount2: U256 = 50.into();
    let data2: String = "0x00".to_string();
    instance.mint(owner, Key::Account(to), id2, amount2, data2);
    let approved: bool = true;
    instance.set_approval_for_all(owner, Key::Account(to), approved);
    let from = to;
    let transfer_to = Key::Account(owner);
    let data: String = "0x00".to_string();
    let ids: Vec<U256> = vec![id1, id2];
    let amounts: Vec<U256> = vec![20.into(), 10.into()];

    instance.safe_batch_transfer_from(from, Key::Account(from), transfer_to, ids, amounts, data);

    TestContract::new(
        &env,
        "erc1155-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF),
            "package_hash" => Key::Hash(instance.package_hash()),
            "owner"=>Key::from(owner),
            "account"=>Key::Account(from),
            "id"=>id1
        },
        0,
    );
    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, 80.into());
    TestContract::new(
        &env,
        "erc1155-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF),
            "package_hash" => Key::Hash(instance.package_hash()),
            "owner"=>Key::from(owner),
            "account"=>transfer_to,
            "id"=>id1
        },
        0,
    );
    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, 20.into());

    TestContract::new(
        &env,
        "erc1155-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF),
            "package_hash" => Key::Hash(instance.package_hash()),
            "owner"=>Key::from(owner),
            "account"=>Key::Account(from),
            "id"=>id2
        },
        0,
    );
    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, 40.into());
    TestContract::new(
        &env,
        "erc1155-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF),
            "package_hash" => Key::Hash(instance.package_hash()),
            "owner"=>Key::from(owner),
            "account"=>Key::Account(from),
            "id"=>id2
        },
        0,
    );
    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, 40.into());
    TestContract::new(
        &env,
        "erc1155-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF),
            "package_hash" => Key::Hash(instance.package_hash()),
            "owner"=>Key::from(owner),
            "account"=>transfer_to,
            "id"=>id2
        },
        0,
    );
    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, 10.into());
}
#[test]
fn test_burn() {
    let (env, owner, instance) = deploy();
    let instance = MOCKCONTRACTInstance::contract_instance(instance);
    let to: Key = Key::Account(owner);
    let id: U256 = 1.into();
    let amount: U256 = 100.into();
    let data: String = "0x00".to_string();
    instance.mint(owner, to, id, amount, data);
    instance.burn(owner, to, id, 50.into());
    let arg_token_id: U256 = 1.into();
    let arg_owner: Key = Key::Account(owner);
    TestContract::new(
        &env,
        "erc1155-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF),
            "package_hash" => Key::Hash(instance.package_hash()),
            "owner"=>Key::from(owner),
            "account"=>arg_owner,
            "id"=>arg_token_id
        },
        0,
    );
    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, 50.into());
}
#[test]
fn test_burn_batch() {
    let (env, owner, instance) = deploy();
    let instance = MOCKCONTRACTInstance::contract_instance(instance);
    let to: Key = Key::Account(owner);
    let id: U256 = 1.into();
    let amount: U256 = 100.into();
    let data: String = "0x00".to_string();
    instance.mint(owner, to, id, amount, data);
    let id2: U256 = 2.into();
    let amount2: U256 = 50.into();
    let data2: String = "0x00".to_string();

    instance.mint(owner, to, id2, amount2, data2);
    let myids: Vec<U256> = vec![id, id2];
    let amounts: Vec<U256> = vec![20.into(), 10.into()];
    instance.burn_batch(owner, to, myids, amounts);

    let arg_token_id: U256 = 1.into();
    let arg_owner: Key = to;
    TestContract::new(
        &env,
        "erc1155-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF),
            "package_hash" => Key::Hash(instance.package_hash()),
            "owner"=>Key::from(owner),
            "account"=>arg_owner,
            "id"=>arg_token_id
        },
        0,
    );
    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, 80.into());
    let token_id2: U256 = 2.into();
    TestContract::new(
        &env,
        "erc1155-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF),
            "package_hash" => Key::Hash(instance.package_hash()),
            "owner"=>Key::from(owner),
            "account"=>arg_owner,
            "id"=>token_id2
        },
        0,
    );
    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, 40.into());
}
