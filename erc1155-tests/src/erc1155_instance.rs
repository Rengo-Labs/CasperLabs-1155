use casper_types::{
    account::AccountHash,
    bytesrepr::{Bytes, FromBytes},
    runtime_args, CLTyped, Key, RuntimeArgs, U256,
};
use casperlabs_test_env::{TestContract, TestEnv};

pub struct ERC1155Instance(TestContract);

impl ERC1155Instance {
    pub fn contract_instance(contract: TestContract) -> ERC1155Instance {
        ERC1155Instance(contract)
    }
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        uri: String,
    ) -> TestContract {
        TestContract::new(
            env,
            "erc1155-token.wasm",
            contract_name,
            sender,
            runtime_args! {
                "uri" => uri
            },
            0,
        )
    }
    pub fn balance_of(&self, sender: AccountHash, account: Key, id: U256) {
        self.0.call_contract(
            sender,
            "balance_of",
            runtime_args! {
                "account"=>account,
                "id"=>id

            },
            0,
        );
    }
    pub fn uri(&self, sender: AccountHash) {
        self.0.call_contract(sender, "uri", runtime_args! {}, 0);
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
