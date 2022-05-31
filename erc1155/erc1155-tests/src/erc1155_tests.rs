use blake2::digest::consts::U2;
use casper_types::{
    account::AccountHash, runtime_args, ContractPackageHash, Key, RuntimeArgs, URef, U256, U512,
};
use test_env::{TestContract, TestEnv};

use crate::erc1155_instance::ERC1155Instance;

fn deploy() -> (TestEnv, AccountHash, TestContract,TestContract) 
{
    let env = TestEnv::new();
    let owner = env.next_user();
    let contract = ERC1155Instance::new(&env, "ERC1155", owner, "".to_string());
    let proxy = ERC1155Instance::proxy(
        &env,
        "proxy_test",
        owner,
        Key::Hash(contract.contract_hash()),
    );

    (env, owner, contract,proxy) 
}

#[test]
fn test_deploy() {
    let (_, _, _,_)= deploy();
}
 #[test]
fn test_balance_of() {
    let (env, owner, contract, proxy)
     = deploy();
     let proxy = ERC1155Instance::contract_instance(proxy);
    let arg_token_id: U256 = 1.into();
    let arg_owner: Key = Key::Account(owner);
    proxy.balance_of(owner, arg_token_id, arg_owner);
    let res:U256= proxy.result();
     assert_eq!(res,1000000000.into());
}
#[test]
fn test_is_approved_for_all() {
    let (env, owner, contract, proxy)
     = deploy();
     let proxy = ERC1155Instance::contract_instance(proxy);
    let arg_account: Key = Key::Account(owner);
    let arg_operator:Key=Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap();
    proxy.is_approved_for_all(owner,arg_account,arg_operator);
    let res:bool= proxy.result();
     assert_eq!(res,true);
}
#[test]
fn test_set_approval_for_all() {
    let (env, owner, contract, proxy)
     = deploy();
     let contract = ERC1155Instance::contract_instance(contract);
    let arg_operator:Key=Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap();
    contract.set_approval_for_all(owner,arg_operator,true);
    
}


