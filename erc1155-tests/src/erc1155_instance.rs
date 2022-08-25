use casper_types::{
    account::AccountHash, bytesrepr::{FromBytes, Bytes}, runtime_args, CLTyped, ContractPackageHash, Key,
    RuntimeArgs, URef, U256,
};
use casperlabs_test_env::{TestContract, TestEnv};

pub struct ERC1155Instance(TestContract);

impl ERC1155Instance {
    pub fn contract_instance(contract: TestContract) -> ERC1155Instance {
        ERC1155Instance(contract)
    }
    pub fn new(env: &TestEnv, contract_name: &str, sender: AccountHash, uri: String) -> TestContract {
        TestContract::new(
            env,
            "erc1155-token.wasm",
            contract_name,
            sender,
            runtime_args! {
                "uri" => uri
            },
            0
            
        )
    }
    pub fn proxy(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        erc1155: Key,
    ) -> TestContract {
        TestContract::new(
            env,
            "erc1155-proxy-token.wasm",
            contract_name,
            sender,
            runtime_args! {
                "erc1155" => erc1155,
            },
            0
        )
        
    }
    pub fn balance_of(&self, sender: AccountHash, token_id: U256, owner: Key) {
        self.0.call_contract(
            sender,
            "balance_of",
            runtime_args! {
                "token_id"=>token_id,
                "owner"=>owner

            },
            0
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
            0
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
            0
        );
    }

    pub fn safe_transfer_from(
        &self,
        sender: AccountHash,
        from: Key,
        to: Key,
        id: U256,
        amount: U256,
        _data: Bytes,
    ) {
        self.0.call_contract(
            sender,
            "safe_transfer_from",
            runtime_args! {

                "from"=>from,
               "to"=>  to,
               "id"=>id,
               "amount"=>amount,
               "_data"=>_data
            },
            0
        );
    }
    pub fn safe_batch_transfer_from(
        &self,
        sender: AccountHash,
        from: Key,
        to: Key,
        ids: Vec<U256>,
        amounts: Vec<U256>,
        _data: Bytes,
    ) {
        self.0.call_contract(
            sender,
            "safe_transfer_from",
            runtime_args! {

            "from"=>from,
               "to"=>  to,
               "ids"=>ids,
               "amounts"=>amounts,
               "_data"=>_data
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
