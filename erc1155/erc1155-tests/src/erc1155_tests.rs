use blake2::digest::consts::U2;
use casper_types::{
    account::AccountHash, runtime_args, ContractPackageHash, Key, RuntimeArgs, URef, U256, U512,
};
use test_env::{TestContract, TestEnv};

use crate::erc1155_instance::ERC1155Instance;

fn deploy() -> (TestEnv, AccountHash, TestContract) 
{
    let env = TestEnv::new();
    let owner = env.next_user();
    let contract = ERC1155Instance::new(&env, "ERC1155", owner, "".to_string());
    
    (env, owner, contract) 
}

#[test]
fn test_deploy() {
    let (_, _, _)= deploy();
}

