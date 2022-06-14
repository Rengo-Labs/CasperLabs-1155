#![no_main]
#![no_std]

extern crate alloc;
use alloc::{boxed::Box, collections::BTreeSet, format, vec, vec::Vec, string::String};

use casper_contract::{
    contract_api::{account, runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},
    runtime_args, CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints,
    Group, Key, Parameter, RuntimeArgs, URef, U256, CLValue, system::handle_payment::RuntimeProvider,
};
pub mod mappings;

#[no_mangle]
fn constructor() {
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    let erc1155: Key = runtime::get_named_arg("erc1155");
    
    mappings::set_key(&mappings::self_hash_key(), contract_hash);
    mappings::set_key(&mappings::self_package_key(), package_hash);
    mappings::set_key(
        &mappings::erc1155_key(),
        ContractHash::from(erc1155.into_hash().unwrap_or_default()),
    );
}

#[no_mangle]
fn balance_of(){
    let erc1155_address: ContractHash =
        mappings::get_key(&mappings::erc1155_key());
        let token_id:U256 = runtime::get_named_arg("token_id");
        let owner:Key = runtime::get_named_arg("owner");
        let ret:U256 = runtime::call_contract(
            erc1155_address,
            "balance_of",
            runtime_args! {
                "token_id" =>token_id,
                "owner" => owner
            },
        );
       mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn balance_of_batch(){
    let erc1155_address: ContractHash =
        mappings::get_key(&mappings::erc1155_key());
        let accounts:Vec<Key> = runtime::get_named_arg("accounts");
        let ids:Vec<U256> = runtime::get_named_arg("ids");
        let ret:U256 = runtime::call_contract(
            erc1155_address,
            "balance_of_batch",
            runtime_args! {
                "accounts" =>accounts,
                "ids" => ids
            },
        );
       mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn is_approved_for_all(){
    let erc1155_address: ContractHash =
        mappings::get_key(&mappings::erc1155_key());
        let account:Key = runtime::get_named_arg("account");
        let operator:Key = runtime::get_named_arg("operator");
        let ret:bool = runtime::call_contract(
            erc1155_address,
            "is_approved_for_all",
            runtime_args! {
                "account" =>account,
                "operator" => operator
            },
        );
       mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("erc1155", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "balance_of",
        vec![
            Parameter::new("token_id", U256::cl_type()),
            Parameter::new("owner", Key::cl_type())
           
            
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "balance_of_batch",
        vec![
            Parameter::new("accounts", Vec::<Key>::cl_type()),
            Parameter::new("ids", Vec::<U256>::cl_type()),
            
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "is_approved_for_all",
        vec![
            Parameter::new("account", Key::cl_type()),
            Parameter::new("operator", Key::cl_type()),
            
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
   
   
    entry_points
}

#[no_mangle]
fn call() {
    // Build new package with initial a first version of the contract.
    let (package_hash, access_token) = storage::create_contract_package_at_hash();
    let (contract_hash, _) =
        storage::add_contract_version(package_hash, get_entry_points(), Default::default());
    let erc1155: Key = runtime::get_named_arg("erc1155");

    // Prepare constructor args
    let constructor_args = runtime_args! {
        "contract_hash" => contract_hash,
        "package_hash" => package_hash,
        "erc1155" => erc1155
    };

    // Add the constructor group to the package hash with a single URef.
    let constructor_access: URef =
        storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
            .unwrap_or_revert()
            .pop()
            .unwrap_or_revert();

    // Call the constructor entry point
    let _: () =
        runtime::call_versioned_contract(package_hash, None, "constructor", constructor_args);

    // Remove all URefs from the constructor group, so no one can call it for the second time.
    let mut urefs = BTreeSet::new();
    urefs.insert(constructor_access);
    storage::remove_contract_user_group_urefs(package_hash, "constructor", urefs)
        .unwrap_or_revert();

    // Store contract in the account's named keys.
    let contract_name: alloc::string::String = runtime::get_named_arg("contract_name");
    runtime::put_key(
        &format!("{}_package_hash", contract_name),
        package_hash.into(),
    );
    runtime::put_key(
        &format!("{}_package_hash_wrapped", contract_name),
        storage::new_uref(package_hash).into(),
    );
    runtime::put_key(
        &format!("{}_contract_hash", contract_name),
        contract_hash.into(),
    );
    runtime::put_key(
        &format!("{}_contract_hash_wrapped", contract_name),
        storage::new_uref(contract_hash).into(),
    );
    runtime::put_key(
        &format!("{}_package_access_token", contract_name),
        access_token.into(),
    );
}