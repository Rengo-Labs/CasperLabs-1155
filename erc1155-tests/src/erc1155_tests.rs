use crate::erc1155_instance::ERC1155Instance;
use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs};
use casperlabs_test_env::{TestContract, TestEnv};
use common::keys::*;
fn deploy() -> (TestEnv, AccountHash, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let contract = ERC1155Instance::new(&env, "ERC1155", owner, "".to_string());
    (env, owner, contract)
}

#[test]
fn test_deploy() {
    let (_, _, _) = deploy();
}
#[test]
fn test_uri() {
    let (env, owner, contract) = deploy();
    let contract = ERC1155Instance::contract_instance(contract);
    TestContract::new(
        &env,
        "erc1155-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(URI),
            "package_hash" => Key::Hash(contract.package_hash()),
            "owner"=>Key::from(owner),
        },
        0,
    );
    let ret: String = env.query_account_named_key(owner, &[URI.into()]);
    assert_eq!(ret, "".to_string());
}
#[test]
fn test_is_approved_for_all() {
    let (_, owner, contract) = deploy();
    let contract = ERC1155Instance::contract_instance(contract);
    let arg_account: Key = Key::Account(owner);
    let arg_operator: Key = Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap();
    contract.is_approved_for_all(owner, arg_account, arg_operator);
}
#[test]
fn test_set_approval_for_all() {
    let (_, owner, contract) = deploy();
    let contract = ERC1155Instance::contract_instance(contract);
    let operator: Key = Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap();
    contract.set_approval_for_all(owner, operator, true);
}
