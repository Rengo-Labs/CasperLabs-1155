#![no_main]
#![no_std]

extern crate alloc;
use alloc::{boxed::Box, collections::BTreeSet, format, string::String, vec, vec::Vec};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::{Bytes, ToBytes},
    runtime_args, CLType, CLTyped, CLValue, ContractHash, ContractPackageHash, EntryPoint,
    EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U256,
};
use casperlabs_contract_utils::{ContractContext, OnChainContractStorage};
use erc1155_crate::ERC1155;

#[derive(Default)]
struct Token(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for Token {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl ERC1155<OnChainContractStorage> for Token {}

impl Token {
    fn constructor(
        &mut self,
        uri: String,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        ERC1155::init(self, uri, Key::from(contract_hash), package_hash);
    }
}

#[no_mangle]
fn constructor() {
    let uri: String = runtime::get_named_arg("uri");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    Token::default().constructor(uri, contract_hash, package_hash);
}

/// This function is to transfer tokens against the address that user provided
///
/// # Parameters
///
/// * `recipient` - A Key that holds the account address of the user
///
/// * `amount` - A U256 that holds the amount for transfer
///  

#[no_mangle]
fn balance_of() {
    let token_id: U256 = runtime::get_named_arg("token_id");
    let owner: Key = runtime::get_named_arg("owner");
    let ret: U256 = Token::default().balance_of(token_id, owner);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn balance_of_batch() {
    let accounts: Vec<Key> = runtime::get_named_arg("accounts");
    let ids: Vec<U256> = runtime::get_named_arg("ids");
    let ret: Vec<U256> = Token::default().balance_of_batch(accounts, ids);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn set_approval_for_all() {
    let operator: Key = runtime::get_named_arg("operator");
    let approved: bool = runtime::get_named_arg("approved");
    Token::default().set_approval_for_all(operator, approved);
}
#[no_mangle]
fn is_approved_for_all() {
    let account: Key = runtime::get_named_arg("account");
    let operator: Key = runtime::get_named_arg("operator");
    let ret: bool = Token::default().is_approved_for_all(account, operator);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn safe_transfer_from() {
    let from: Key = runtime::get_named_arg("from");
    let to: Key = runtime::get_named_arg("to");
    let id: U256 = runtime::get_named_arg("id");
    let amount: U256 = runtime::get_named_arg("amount");
    let data: String = runtime::get_named_arg("data");
    Token::default().safe_transfer_from(from, to, id, amount, data);
}
#[no_mangle]
fn safe_batch_transfer_from() {
    let from: Key = runtime::get_named_arg("from");
    let to: Key = runtime::get_named_arg("to");
    let ids: Vec<U256> = runtime::get_named_arg("ids");
    let amounts: Vec<U256> = runtime::get_named_arg("amounts");
    let data: String = runtime::get_named_arg("data");
    Token::default().safe_batch_transfer_from(from, to, ids, amounts, data);
}
fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("uri", String::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "balance_of",
        vec![
            Parameter::new("token_id", U256::cl_type()),
            Parameter::new("owner", Key::cl_type()),
        ],
      
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "balance_of_batch",
        vec![
            Parameter::new("accounts", CLType::List(Box::new(CLType::Key))),
            Parameter::new("ids", CLType::List(Box::new(CLType::U256))),
        ],
        CLType::List(Box::new(CLType::U256)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_approval_for_all",
        vec![
            Parameter::new("operator", Key::cl_type()),
            Parameter::new("approved", bool::cl_type()),
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
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "safe_transfer_from",
        vec![
            Parameter::new("from", Key::cl_type()),
            Parameter::new("to", Key::cl_type()),
            Parameter::new("id", U256::cl_type()),
            Parameter::new("amount", U256::cl_type()),
            Parameter::new("data", String::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "safe_batch_transfer_from",
        vec![
            Parameter::new("from", Key::cl_type()),
            Parameter::new("to", Key::cl_type()),
            Parameter::new("ids", CLType::List(Box::new(CLType::U256))),
            Parameter::new("amounts", CLType::List(Box::new(CLType::U256))),
            Parameter::new("data", String::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}

#[no_mangle]
fn call() {
    let contract_name: alloc::string::String = runtime::get_named_arg("contract_name");
    if !runtime::has_key(&format!("{}_package_hash", contract_name)) {
        // Build new package with initial a first version of the contract.
        let (package_hash, access_token) = storage::create_contract_package_at_hash();
        let (contract_hash, _) =
            storage::add_contract_version(package_hash, get_entry_points(), Default::default());

        let uri: String = runtime::get_named_arg("uri");

        // Prepare constructor args
        let constructor_args = runtime_args! {
            "uri" => uri,
            "contract_hash" => contract_hash,
            "package_hash"=> package_hash

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
    } else {
        let package_hash: ContractPackageHash =
            runtime::get_key(&format!("{}_package_hash", contract_name))
                .unwrap_or_revert()
                .into_hash()
                .unwrap()
                .into();

        let (contract_hash, _): (ContractHash, _) =
            storage::add_contract_version(package_hash, get_entry_points(), Default::default());

        // update contract hash
        runtime::put_key(
            &format!("{}_contract_hash", contract_name),
            contract_hash.into(),
        );
        runtime::put_key(
            &format!("{}_contract_hash_wrapped", contract_name),
            storage::new_uref(contract_hash).into(),
        );
    }
}
