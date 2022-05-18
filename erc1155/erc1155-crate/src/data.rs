use alloc::string::String;
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    ContractPackageHash, Key, U256,
};
use contract_utils::Dict;
use contract_utils::{get_key, set_key};

pub const SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const SELF_PACKAGE_HASH: &str = "self_package_hash";
pub const RESULT: &str = "result";
pub const OWNERS: &str = "owners";
pub const NAME: &str = "name";
pub const SYMBOL: &str = "symbol";

pub const BALACNES: &str = "balances";
pub struct Balances {
    dict: Dict,
}

impl Balances {
    pub fn instance() -> Balances {
        Balances {
            dict: Dict::instance(BALACNES),
        }
    }

    pub fn init() {
        Dict::init(BALACNES)
    }

    // A bit change is done in set/get by keys (Using templates)

    pub fn get(&self, token_id: &U256, owner: &Key) -> U256 {
        self.dict.get_by_keys((token_id, owner)).unwrap_or_default()
    }

    pub fn set(&self, token_id: &U256, owner: &Key, value: U256) {
        self.dict.set_by_keys((token_id, owner), value);
    }
}
pub const OPERATOR_APPROVALS: &str = "Operator_Approvals";

pub struct OperatorApprovals {
    dict: Dict,
}

impl OperatorApprovals {
    pub fn instance() -> OperatorApprovals {
        OperatorApprovals {
            dict: Dict::instance(OPERATOR_APPROVALS),
        }
    }

    pub fn init() {
        Dict::init(OPERATOR_APPROVALS)
    }

    pub fn get(&self, account: &Key, operator: &Key) -> bool {
        self.dict
            .get_by_keys((account, operator))
            .unwrap_or_default()
    }

    pub fn set(&self, account: &Key, operator: &Key, value: bool) {
        self.dict.set_by_keys((account, operator), value);
    }
}

pub fn ZERO_ADDRESS() -> Key {
    Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap()
}
pub fn uri() -> String {
    get_key("URI").unwrap_or_revert()
}

pub fn set_uri(uri: String) {
    set_key("URI", uri);
}

pub fn set_hash(contract_hash: Key) {
    set_key(SELF_CONTRACT_HASH, contract_hash);
}

pub fn get_hash() -> Key {
    get_key(SELF_CONTRACT_HASH).unwrap_or_revert()
}

pub fn set_package_hash(package_hash: ContractPackageHash) {
    set_key(SELF_PACKAGE_HASH, package_hash);
}

pub fn get_package_hash() -> ContractPackageHash {
    get_key(SELF_PACKAGE_HASH).unwrap_or_revert()
}
