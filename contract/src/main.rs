#![no_main]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(non_snake_case)]

extern crate alloc;

use alloc::{
    collections::{BTreeMap, BTreeSet},
    string::String,
};
use core::convert::TryInto;

use contract::{
    contract_api::runtime::revert,
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};

use types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys},
    runtime_args, ApiError, CLType, CLTyped, CLValue, Group, Parameter, RuntimeArgs, URef, U256
};

#[repr(u16)]
enum FailureCode {
    Zero = 0, // 65,536 as an ApiError::User
    One,      // 65,537 as an ApiError::User
    Two,      // 65,538 as an ApiError::User
}

impl From<FailureCode> for ApiError {
    fn from(code: FailureCode) -> Self {
        ApiError::User(code as u16)
    }
}

#[no_mangle]
pub extern "C" fn name() {
    let val: String = get_key("name");
    ret(val)
}

#[no_mangle]
pub extern "C" fn symbol() {
    let val: String = get_key("symbol");
    ret(val)
}


#[no_mangle]
pub extern "C" fn contract_owner() {
    let val: AccountHash = get_key("owner");
    ret(val)
}
#[no_mangle]
pub extern "C" fn is_owner() {
    let val: AccountHash = get_key("owner");
    let owner: AccountHash = runtime::get_caller();
    let result: bool = (val == owner);
    ret(result)
}
#[no_mangle]
pub extern "C" fn balance_of() {
    let account: AccountHash = runtime::get_named_arg("account");
    let id: U256 = runtime::get_named_arg("id");
    let val: U256 = get_key(&balance_key(&account,&id));
    ret(val)
}
#[no_mangle]
pub extern "C" fn balance_of_batch() {
    let accounts: Vec<AccountHash> = runtime::get_named_arg("accounts");
    let ids: Vec<U256> = runtime::get_named_arg("ids");
    let mut results=Vec::new();
    for i in 0..ids.len(){
        let val: U256 = get_key(&balance_key(&accounts[i],&ids[i]));
        results.push(val);
    }
    
    ret(results)
}
#[no_mangle]
pub extern "C" fn tokenURI() {
    let val: String = get_key("base_uri");
    let id: U256 = runtime::get_named_arg("id");
    ret(uri_formatter(val, id))
}

#[no_mangle]
pub extern "C" fn approval_for_all() {
    let owner: AccountHash = runtime::get_named_arg("owner");
    let spender: AccountHash = runtime::get_named_arg("spender");
    let val: U256 = get_key(&approval_all_key(&owner, &spender));
    ret(val)
}

#[no_mangle]
pub extern "C" fn approve_all() {
    let spender: AccountHash = runtime::get_named_arg("spender");

    _approve_all(runtime::get_caller(), spender);
}
#[no_mangle]
pub extern "C" fn add_minter() {
    let minter: AccountHash = runtime::get_named_arg("minter");
    let is_minter: bool = get_key(&minter_key(&runtime::get_caller()));
    if (is_minter) {
        _add_minter(minter);
    } else {
        revert(ApiError::User(1));
    }
}


#[no_mangle]
pub extern "C" fn mint() {
    let recipient: AccountHash = runtime::get_named_arg("to");
    let id: U256 = runtime::get_named_arg("id");
    let amount: U256 = runtime::get_named_arg("amount");
    let caller: AccountHash = runtime::get_caller();
    let is_minter: bool = get_key(&minter_key(&caller));
    if(is_minter==true){
        _mint(id, recipient,amount );
    }else{
        revert(ApiError::User(1))
    }
    
}

#[no_mangle]
pub extern "C" fn batch_mint() {
    let recipient: AccountHash = runtime::get_named_arg("to");
    let ids: Vec<U256> = runtime::get_named_arg("ids");
    let values: Vec<U256> = runtime::get_named_arg("values");
    let caller: AccountHash = runtime::get_caller();
    let is_minter: bool = get_key(&minter_key(&caller));
    if(is_minter==true){
        _batch_mint(ids,values, recipient);
    }else{
        revert(ApiError::User(1))
    }
    
}

#[no_mangle]
pub extern "C" fn safe_transfer_from() {
    let owner: AccountHash = runtime::get_named_arg("owner");
    let recipient: AccountHash = runtime::get_named_arg("recipient");
    let id: U256 = runtime::get_named_arg("id");
    let value: U256 = runtime::get_named_arg("amount");
    _transfer_from(owner, recipient, id,value);
}

#[no_mangle]
pub extern "C" fn safe_batch_transfer_from() {
    let owner: AccountHash = runtime::get_named_arg("owner");
    let recipient: AccountHash = runtime::get_named_arg("recipient");
    let ids: Vec<U256> = runtime::get_named_arg("ids");
    let values: Vec<U256> = runtime::get_named_arg("values");
    _batch_transfer_from(owner, recipient, ids,values);
}

#[no_mangle]
pub extern "C" fn call() {
    let token_name: String = runtime::get_named_arg("token_name");
    let token_symbol: String = runtime::get_named_arg("token_symbol");
    let base_uri: String = runtime::get_named_arg("base_uri");
    
    let owner: AccountHash = runtime::get_named_arg("owner");

    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(endpoint("name", vec![], CLType::String));
    entry_points.add_entry_point(endpoint("symbol", vec![], CLType::String));
    entry_points.add_entry_point(endpoint("symbol", vec![], CLType::String));
   
   
    entry_points.add_entry_point(endpoint(
        "balance_of",
        vec![Parameter::new("account", AccountHash::cl_type()),
         Parameter::new("id",CLType::U256)],
        CLType::U256,
    ));
    entry_points.add_entry_point(endpoint(
        "balance_of_batch",
        vec![
        Parameter::new("accounts", CLType::List(Box::new(AccountHash::cl_type()))),
        Parameter::new("ids",CLType::List(Box::new(CLType::U256))) ],
        CLType::List(Box::new(CLType::U256))
    ));
    
    entry_points.add_entry_point(endpoint(
        "approve_all" ,vec![
            Parameter::new("owner", AccountHash::cl_type()),
            Parameter::new("spender", AccountHash::cl_type()),
        ],
        CLType::Unit,
    ));
 
    entry_points.add_entry_point(endpoint(
        "approval_for_all",
        vec![
            Parameter::new("owner", AccountHash::cl_type()),
            Parameter::new("spender", AccountHash::cl_type()),
        ],
        CLType::Bool,
    ));
    
    entry_points.add_entry_point(endpoint(
        "mint",
        vec![
            Parameter::new("to", AccountHash::cl_type()),
            Parameter::new("id", CLType::U256),
            Parameter::new("amount", CLType::U256)
        ],
        CLType::Unit,
    ));
       
    entry_points.add_entry_point(endpoint(
        "batch_mint",
        vec![
            Parameter::new("to", AccountHash::cl_type()),
            Parameter::new("ids", CLType::List(Box::new(CLType::U256))),
            Parameter::new("values", CLType::List(Box::new(CLType::U256)))
        ],
        CLType::Unit,
    ));
    entry_points.add_entry_point(endpoint(
        "add_minter",
        vec![Parameter::new("minter", AccountHash::cl_type())],
        CLType::Unit,
    ));
    entry_points.add_entry_point(endpoint(
        "safe_transfer_from",
        vec![
            Parameter::new("owner", AccountHash::cl_type()),
            Parameter::new("recipient", AccountHash::cl_type()),
            Parameter::new("id", CLType::U256),
            Parameter::new("amount", CLType::U256),
        ],
        CLType::Unit,
    ));
    entry_points.add_entry_point(endpoint(
        "safe_batch_transfer_from",
        vec![
            Parameter::new("owner", AccountHash::cl_type()),
            Parameter::new("recipient", AccountHash::cl_type()),
            Parameter::new("ids",CLType::List(Box::new(CLType::U256))) ,
            Parameter::new("amounts",CLType::List(Box::new(CLType::U256))) ,
        ],
        CLType::Unit,
    ));
    let mut named_keys = NamedKeys::new();
    named_keys.insert("name".to_string(), storage::new_uref(token_name).into());
    named_keys.insert("symbol".to_string(), storage::new_uref(token_symbol).into());
    named_keys.insert("base_uri".to_string(), storage::new_uref(base_uri).into());
    named_keys.insert(
        "contract_owner".to_string(),
        storage::new_uref(owner).into(),
    );
   
   
    named_keys.insert(
        minter_key(&runtime::get_caller()),
        storage::new_uref(true).into(),
    );
    let (contract_hash, _) =
        storage::new_locked_contract(entry_points, Some(named_keys), None, None);
    runtime::put_key("ERC1155", contract_hash.into());
    runtime::put_key("ERC1155_hash", storage::new_uref(contract_hash).into());
}

fn _transfer(sender: AccountHash, recipient: AccountHash, id: U256,value:U256) {
    let sender_key = balance_key(&sender,&id);
    let recipient_key = balance_key(&recipient,&id);
   
    let account_default: AccountHash = Default::default();
    if( get_key::<U256>(&sender_key) <value ){
        revert(ApiError::User(2));
    }
    let new_sender_balance: U256 = (get_key::<U256>(&sender_key)) - value;
    set_key(&sender_key, new_sender_balance);
    let new_recipient_balance: U256 = (get_key::<U256>(&recipient_key)) + value;
    set_key(&recipient_key, new_recipient_balance);
    
    set_key(&approval_key( &id),account_default);
}

fn _transfer_from(owner: AccountHash, recipient: AccountHash, id: U256,amount:U256) {
    let sender = runtime::get_caller();    
    let approve_all = approval_all_key(&owner, &sender);    
    
    let all_true: bool = get_key::<bool>(&approve_all);
    if all_true || owner == sender {
        _transfer(owner, recipient, id,amount);
    }
}
fn _batch_transfer_from(owner: AccountHash, recipient: AccountHash,ids: Vec<U256>,values:Vec<U256>) {
    let sender = runtime::get_caller();    
    let approve_all = approval_all_key(&owner, &sender);    
    
    let all_true: bool = get_key::<bool>(&approve_all);
    if all_true || owner == sender {
        _batch_transfer(owner, recipient, ids,values);
    }else{
        revert(ApiError::User(1));
    } 
}
fn _mint(id: U256, receiver: AccountHash,value:U256) {    
      
        let balance_key = balance_key(&receiver,&id);
        let new_recipient_balance: U256 = (get_key::<U256>(&balance_key) + value);
        set_key(&balance_key, new_recipient_balance);           
   
}
fn _batch_transfer(sender:AccountHash,receiver: AccountHash,ids: Vec<U256>,values:Vec<U256>, ){
    if(ids.len()==values.len()){
        for i in 0..ids.len(){
            _transfer(sender, receiver, ids[i],values[i]);
        }

    } else{
        revert(ApiError::User(1));
    }     
}
fn _batch_mint(ids: Vec<U256>,values:Vec<U256>, receiver: AccountHash) {
  
  
        if(ids.len()==values.len()){
            for i in 0..ids.len(){
                _mint(ids[i], receiver,values[i]);
            }

        }else{
            revert(ApiError::User(2));
        }      
              
    
}


fn _approve_all(owner: AccountHash, spender: AccountHash) {
    set_key(&approval_all_key(&owner, &spender), true);
}
fn _add_minter(minter: AccountHash) {
    set_key(&minter_key(&minter), true);
}
fn _remove_minter(minter: AccountHash) {
    set_key(&minter_key(&minter), false);
}
fn ret<T: CLTyped + ToBytes>(value: T) {
    runtime::ret(CLValue::from_t(value).unwrap_or_revert())
}

fn get_key<T: FromBytes + CLTyped + Default>(name: &str) -> T {
    match runtime::get_key(name) {
        None => Default::default(),
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            storage::read(key).unwrap_or_revert().unwrap_or_revert()
        }
    }
}

fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
    match runtime::get_key(name) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(name, key);
        }
    }
}

fn balance_key(account: &AccountHash,id:&U256) -> String {
    format!("balances_{}_{}", account,id)
}
fn minter_key(account: &AccountHash) -> String {
    format!("minter_{}", account)
}

fn uri_formatter(base: String, id: U256) -> String {
    format!("{}{}", base, id)
}
fn approval_key( id: &U256) -> String {
    format!("approval_{}", id)
}
fn approval_all_key(owner: &AccountHash, spender: &AccountHash) -> String {
    format!("approval_all_{}_{}", owner, spender)
}

fn endpoint(name: &str, param: Vec<Parameter>, ret: CLType) -> EntryPoint {
    EntryPoint::new(
        String::from(name),
        param,
        ret,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}
