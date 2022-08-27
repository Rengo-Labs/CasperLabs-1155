use crate::data;
use alloc::{string::String, vec::Vec};
use casper_types::{bytesrepr::Bytes, ContractPackageHash, Key, U256};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use erc1155_crate::ERC1155;
pub trait MOCKCONTRACT<Storage: ContractStorage>:
    ContractContext<Storage> + ERC1155<Storage>
{
    fn init(&mut self, uri: String, contract_hash: Key, package_hash: ContractPackageHash) {
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);
        ERC1155::init(self, uri, contract_hash, package_hash);
    }
    fn mint(&mut self, to: Key, id: U256, amount: U256, data: String) {
        let _data: Bytes = Bytes::from(data.as_bytes());
        ERC1155::_mint(self, to, id, amount, _data);
    }
    fn burn(&mut self, from: Key, id: U256, amount: U256) {
        ERC1155::_burn(self, from, id, amount);
    }
    fn burn_batch(&mut self, from: Key, ids: Vec<U256>, amounts: Vec<U256>) {
        ERC1155::_burn_batch(self, from, ids, amounts);
    }
}
