
use casper_engine_test_support::{TestContext, AccountHash};
use casper_types::{
    bytesrepr::FromBytes, runtime_args, CLTyped, ContractPackageHash, Key, RuntimeArgs, URef, U256,
};
use test_env::{Sender, TestContract, TestEnv};

pub struct ERC1155Instance(TestContract);

impl ERC1155Instance {
    
    pub fn contract_instance(contract:TestContract)->ERC1155Instance{
        ERC1155Instance(contract)
    }
    pub fn new(env: &TestEnv, contract_name: &str, sender: Sender,name:String,symbol:String) -> TestContract {
        TestContract::new(
            env,
            "erc1155-token.wasm",
            contract_name,
            sender,
            runtime_args! {
                "name" => name,
                "symbol" => symbol,
            },
            0,
        )
    }
    pub fn proxy(env: &TestEnv, contract_name: &str, sender: Sender,erc1155:Key) -> TestContract {
        TestContract::new(
             env,
             "contract.wasm",
             contract_name,
             sender,
             runtime_args! {
                 "erc1155" => erc1155
             },
             0,
         )
     }
    pub fn balance_of(
        &self,
        sender: Sender,
        owner: Key,
    ) {
        self.0.call_contract(
            sender,
            "balance_of",
            runtime_args! {
                "owner" => owner
            },
            0
        );
    }
    pub fn owner_of(
        &self,
        sender: Sender,
        token_id:U256,
    ) {
        self.0.call_contract(
            sender,
            "owner_of",
            runtime_args! {
                "token_id" => token_id
            },
            0
        );
    }
    pub fn name(
        &self,
        sender: Sender,
    ) {
        self.0.call_contract(
            sender,
            "name",
            runtime_args! {},
            0
        );
    }
    pub fn symbol(
        &self,
        sender: Sender,
    ) {
        self.0.call_contract(
            sender,
            "symbol",
            runtime_args! {},
            0
        );
    }
    pub fn token_uri(
        &self,
        sender: Sender,
        token_id:U256,
    ) {
        self.0.call_contract(
            sender,
            "token_uri",
            runtime_args! {
                "token_id" => token_id
            },
            0
        );
    }
    pub fn approve(
        &self,
        sender: Sender,
        to:Key,
        token_id:U256,
    ) {
        self.0.call_contract(
            sender,
            "approve",
            runtime_args! {
                "to" => to,
                "token_id" => token_id
            },
            0
        );
    }
    pub fn get_approved(
        &self,
        sender: Sender,
        token_id:U256,
    ) {
        self.0.call_contract(
            sender,
            "get_approved",
            runtime_args! {
                "token_id" => token_id
            },
            0
        );
    }
    pub fn set_approved_for_all(
        &self,
        sender: Sender,
        operator:Key,
        approved:bool
    ) {
        self.0.call_contract(
            sender,
            "set_approved_for_all",
            runtime_args! {
                "operator" => operator,
                "approved" => approved,
            },
            0
        );
    }
    pub fn is_approved_for_all(
        &self,
        sender: Sender,
        owner:Key,
        operator:Key,
    ) {
        self.0.call_contract(
            sender,
            "is_approved_for_all",
            runtime_args! {
                "owner" => owner,
                "operator" => operator,
            },
            0
        );
    }
    pub fn transfer_from(
        &self,
        sender: Sender,
        from:Key,
        to:Key,
        token_id:U256,
    ) {
        self.0.call_contract(
            sender,
            "transfer_from",
            runtime_args! {
                "from" => from,
                "to" => to,
                "token_id" => token_id,
            },
            0
        );
    }
    pub fn safe_transfer_from(
        &self,
        sender: Sender,
        from:Key,
        to:Key,
        token_id:U256,
    ) {
        self.0.call_contract(
            sender,
            "safe_transfer_from",
            runtime_args! {
                "from" => from,
                "to" => to,
                "token_id" => token_id,
            },
            0
        );
    }
    pub fn mint(
        &self,
        sender: Sender,
        to:Key,
        token_id:U256,
    ) {
        self.0.call_contract(
            sender,
            "mint",
            runtime_args! {
                "to" => to,
                "token_id" => token_id,
            },
            0
        );
    }
    pub fn burn(
        &self,
        sender: Sender,
        token_id:U256,
    ) {
        self.0.call_contract(
            sender,
            "burn",
            runtime_args! {
                "token_id" => token_id,
            },
            0
        );
    }
    // Result methods
    pub fn result<T: CLTyped + FromBytes>(&self) -> T {
        self.0.query_named_key("result".to_string())
    }

    pub fn package_hash(&self) -> ContractPackageHash {
        self.0.query_named_key("self_package_hash".to_string())
    }
}
