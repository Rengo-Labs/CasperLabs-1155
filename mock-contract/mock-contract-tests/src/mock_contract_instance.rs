use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, Key, RuntimeArgs, U256,
};
use casperlabs_test_env::{TestContract, TestEnv};

pub struct MOCKCONTRACTInstance(TestContract);

impl MOCKCONTRACTInstance {
    pub fn contract_instance(contract: TestContract) -> MOCKCONTRACTInstance {
        MOCKCONTRACTInstance(contract)
    }
    pub fn new_deploy(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        uri: String,
    ) -> TestContract {
        TestContract::new(
            env,
            "mock-contract.wasm",
            contract_name,
            sender,
            runtime_args! {
                "uri" => uri
            },
            0,
        )
    }
    pub fn mint(&self, sender: AccountHash, to: Key, id: U256, amount: U256, data: String) {
        self.0.call_contract(
            sender,
            "mint",
            runtime_args! {
                "to" => to,
                "id" => id,
                "amount" => amount,
                "data" => data,
            },
            0,
        );
    }
    pub fn is_approved_for_all(&self, sender: AccountHash, account: Key, operator: Key) {
        self.0.call_contract(
            sender,
            "is_approved_for_all",
            runtime_args! {
                "account"=>account,
                "operator"=>operator

            },
            0,
        );
    }
    pub fn set_approval_for_all(&self, sender: AccountHash, operator: Key, approved: bool) {
        self.0.call_contract(
            sender,
            "set_approval_for_all",
            runtime_args! {
                "operator"=>operator,
                "approved"=>approved

            },
            0,
        );
    }
    pub fn safe_transfer_from(
        &self,
        sender: AccountHash,
        from: Key,
        to: Key,
        id: U256,
        amount: U256,
        data: String,
    ) {
        self.0.call_contract(
            sender,
            "safe_transfer_from",
            runtime_args! {

                "from"=>from,
               "to"=>  to,
               "id"=>id,
               "amount"=>amount,
               "data"=>data
            },
            0,
        );
    }
    pub fn safe_batch_transfer_from(
        &self,
        sender: AccountHash,
        from: Key,
        to: Key,
        ids: Vec<U256>,
        amounts: Vec<U256>,
        data: String,
    ) {
        self.0.call_contract(
            sender,
            "safe_batch_transfer_from",
            runtime_args! {

            "from"=>from,
               "to"=>  to,
               "ids"=>ids,
               "amounts"=>amounts,
               "data"=>data
            },
            0,
        );
    }
    pub fn burn(&self, sender: AccountHash, from: Key, id: U256, amount: U256) {
        self.0.call_contract(
            sender,
            "burn",
            runtime_args! {
                "from" => from,
                "id" => id,
                "amount" => amount
            },
            0,
        );
    }
    pub fn burn_batch(&self, sender: AccountHash, from: Key, ids: Vec<U256>, amounts: Vec<U256>) {
        self.0.call_contract(
            sender,
            "burn_batch",
            runtime_args! {
                "from" => from,
                "ids" => ids,
                "amounts" => amounts
            },
            0,
        );
    }
    
    // Result methods
    pub fn result<T: CLTyped + FromBytes>(&self) -> T {
        self.0.query_named_key("result".to_string())
    }

    pub fn package_hash(&self) -> [u8; 32] {
        self.0.package_hash()
    }
}
