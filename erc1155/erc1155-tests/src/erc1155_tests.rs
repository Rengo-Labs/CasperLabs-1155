use blake2::digest::consts::U2;
use casper_contract::{contract_api::runtime, ext_ffi::casper_get_caller};
use casper_engine_test_support::AccountHash;
use casper_types::{runtime_args, ContractPackageHash, Key, RuntimeArgs, URef, U256, U512, system::auction::RuntimeProvider, bytesrepr::ToBytes, gens::contract_arb};
use contract_utils::ContractContext;
use renvm_sig::keccak256;
use test_env::{Sender, TestContract, TestEnv};
use crate::erc1155_instance::ERC1155Instance;

fn deploy() -> (TestEnv, AccountHash, TestContract,TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let instance = ERC1155Instance::new(&env, "ERC1155", Sender(owner),"sa".to_string(),"s".to_string());
    let proxy = ERC1155Instance::proxy(&env, "ERC1155PROXY", Sender(owner),Key::Hash(instance.contract_hash()));
    (env, owner, instance,proxy)
}

#[test]
fn test_deploy(){
   let (_, _, _,_) = deploy();
}


