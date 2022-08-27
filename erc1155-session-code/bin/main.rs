#![no_std]
#![no_main]

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;
use alloc::{string::String, vec::Vec};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::ToBytes, runtime_args, ApiError, CLTyped, Key, RuntimeArgs, URef, U256,
};
use common::keys::*;

// Key is the same a destination
fn store<T: CLTyped + ToBytes>(key: &str, value: T) {
    // Store `value` under a new unforgeable reference.
    let value_ref: URef = storage::new_uref(value);

    // Wrap the unforgeable reference in a value of type `Key`.
    let value_key: Key = value_ref.into();

    // Store this key under the name "special_value" in context-local storage.
    runtime::put_key(key, value_key);
}

#[no_mangle]
pub extern "C" fn call() {
    let entrypoint: String = runtime::get_named_arg("entrypoint");
    let package_hash: Key = runtime::get_named_arg("package_hash");
    match entrypoint.as_str() {
        BALANCE_OF => {
            let account: Key = runtime::get_named_arg("account");
            let id: U256 = runtime::get_named_arg("id");
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                BALANCE_OF,
                runtime_args! {
                    "account" => account,
                    "id" => id
                },
            );
            store(BALANCE_OF, ret);
        }
        BALANCE_OF_BATCH => {
            let accounts: Vec<String> = runtime::get_named_arg("accounts");
            let ids: Vec<U256> = runtime::get_named_arg("ids");
            let ret: Vec<U256> = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                BALANCE_OF_BATCH,
                runtime_args! {
                    "accounts" => accounts,
                    "ids" => ids
                },
            );
            store(BALANCE_OF_BATCH, ret);
        }
        IS_APPROVED_FOR_ALL => {
            let account: Key = runtime::get_named_arg("account");
            let operator: Key = runtime::get_named_arg("operator");
            let ret: bool = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                IS_APPROVED_FOR_ALL,
                runtime_args! {
                    "account" => account,
                    "operator" => operator
                },
            );
            store(IS_APPROVED_FOR_ALL, ret);
        }
        URI => {
            let ret: String = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                URI,
                runtime_args! {},
            );
            store(URI, ret);
        }
        _ => runtime::revert(ApiError::UnexpectedKeyVariant),
    };
}
