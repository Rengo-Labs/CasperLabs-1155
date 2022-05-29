use core::ptr::eq;

use crate::data::{self, Balances, OperatorApprovals};
use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::{Bytes, ToBytes},
    runtime_args, ApiError, ContractPackageHash, Key, RuntimeArgs, URef, U256,
};
use contract_utils::{get_key, set_key, ContractContext, ContractStorage};

#[repr(u16)]
pub enum Error {
    InvalidOwner = 0,
    NotOwnerNotApproved,
    TransferToZeroAddress,
    InsufficientBalance,
    MismatchIdsAndLength,
    MintToZeroAddress,
    BurnFromZeroAddress,
    BurnAmountExceedsBal,
    SettingApprovalForSelf,
    MismatchIdsAndAccLength,
    Erc115RejectedTok,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
pub enum ERC1155Event {
    TransferBatch {
        operator: Key,
        from: Key,
        to: Key,
        ids: Vec<U256>,
        amounts: Vec<U256>,
    },
    TransferSingle {
        operator: Key,
        from: Key,
        to: Key,
        id: U256,
        amount: U256,
    },
    ApprovalForAll {
        owner: Key,
        operator: Key,
        approved: bool,
    },
}

impl ERC1155Event {
    pub fn type_name(&self) -> String {
        match self {
            ERC1155Event::TransferBatch {
                operator: _,
                from: _,
                to: _,
                ids: _,
                amounts: _,
            } => "TransferBatch",
            ERC1155Event::TransferSingle {
                operator: _,
                from: _,
                to: _,
                id: _,
                amount: _,
            } => "TransferSingle",
            ERC1155Event::ApprovalForAll {
                owner: _,
                operator: _,
                approved: _,
            } => "approval for all",
        }
        .to_string()
    }
}
pub trait ERC1155<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&self, uri: String, contract_hash: Key, package_hash: ContractPackageHash) {
        data::set_uri(uri);
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);
        Balances::init();
        OperatorApprovals::init();
        Balances::instance().set(&U256::from(1), &self.get_caller(), 2.into());
        OperatorApprovals::instance().set(&self.get_caller(), &data::ZERO_ADDRESS(), true);
    }
    fn balance_of(&self, token_id: U256, owner: Key) -> U256 {
        if !(owner != data::ZERO_ADDRESS()) {
            runtime::revert(ApiError::from(Error::InvalidOwner));
        }
        Balances::instance().get(&token_id, &owner)
    }
    fn balance_of_batch(&self, accounts: Vec<Key>, ids: Vec<U256>) -> Vec<U256> {
        if !(accounts.len() == ids.len()) {
            runtime::revert(ApiError::from(Error::MismatchIdsAndAccLength));
        }
        let mut batch_balances: Vec<U256> = Vec::new();
        let mut current_bal: U256 = U256::from(0);
        for i in 0..ids.len() {
            current_bal = self.balance_of(ids[i], accounts[i]);
            batch_balances.push(current_bal);
        }
        batch_balances
    }
    fn set_approval_for_all(&mut self, operator: Key, approved: bool) {
        self._set_approval_for_all(self.get_caller(), operator, approved);
    }
    fn is_approved_for_all(&mut self, account: Key, operator: Key) -> bool {
        OperatorApprovals::instance().get(&account, &operator)
    }
    fn safe_transfer_from(&mut self, from: Key, to: Key, id: U256, amount: U256, _data: Bytes) {
        if !(from == self.get_caller() || self.is_approved_for_all(from, self.get_caller())) {
            runtime::revert(ApiError::from(Error::NotOwnerNotApproved));
        }
        self._safe_transfer_from(from, to, id, amount, _data);
    }
    fn safe_batch_transfer_from(
        &mut self,
        from: Key,
        to: Key,
        ids: Vec<U256>,
        amounts: Vec<U256>,
        _data: Bytes,
    ) {
        if !(from == self.get_caller() || self.is_approved_for_all(from, self.get_caller())) {
            runtime::revert(ApiError::from(Error::NotOwnerNotApproved));
        }
        self._safe_batch_transfer_from(from, to, ids, amounts, _data);
    }
    fn _safe_transfer_from(&mut self, from: Key, to: Key, id: U256, amount: U256, _data: Bytes) {
        if !(to != data::ZERO_ADDRESS()) {
            runtime::revert(ApiError::from(Error::TransferToZeroAddress));
        }
        let operator: Key = self.get_caller();

        let ids: Vec<U256> = self._asSingletonArray(id);
        let amounts: Vec<U256> = self._asSingletonArray(amount);

        self._before_token_transfer(
            operator,
            from,
            to,
            ids.clone(),
            amounts.clone(),
            _data.clone(),
        );
        let from_balance: U256 = Balances::instance().get(&id, &from);
        if !(from_balance >= amount) {
            runtime::revert(ApiError::from(Error::InsufficientBalance));
        }
        let updated_amount_sender: U256 = from_balance.checked_sub(amount).unwrap_or_revert();
        Balances::instance().set(&id, &from, updated_amount_sender);
        let prev_amount_reciever: U256 = Balances::instance().get(&id, &to);
        let updated_amount_reciever: U256 =
            prev_amount_reciever.checked_add(amount).unwrap_or_revert();

        self.erc1155_emit(&ERC1155Event::TransferSingle {
            operator: operator,
            from: from,
            to: to,
            id: id,
            amount: amount,
        });
        self._after_token_transfer(operator, from, to, ids, amounts, _data.clone());
        self._do_safe_transfer_acceptance_check(operator, from, to, id, amount, _data);
    }

    fn _safe_batch_transfer_from(
        &mut self,
        from: Key,
        to: Key,
        ids: Vec<U256>,
        amounts: Vec<U256>,
        _data: Bytes,
    ) {
        if !(ids.len() == amounts.len()) {
            runtime::revert(ApiError::from(Error::MismatchIdsAndLength));
        }
        if !(to != self.get_caller()) {
            runtime::revert(ApiError::from(Error::TransferToZeroAddress));
        }
        let operator: Key = self.get_caller();
        self._before_token_transfer(
            operator,
            from,
            to,
            ids.clone(),
            amounts.clone(),
            _data.clone(),
        );

        for i in 0..ids.len().clone() {
            let id: U256 = ids[i].clone();
            let amount: U256 = amounts[i].clone();
            let from_balance: U256 = Balances::instance().get(&id, &from);
            if !(from_balance >= amount) {
                runtime::revert(ApiError::from(Error::InsufficientBalance));
            }
            let updated_amount_sender: U256 = from_balance.checked_sub(amount).unwrap_or_revert();
            Balances::instance().set(&id, &from, updated_amount_sender);
            let prev_amount_reciever: U256 = Balances::instance().get(&id, &to);
            let updated_amount_reciever =
                prev_amount_reciever.checked_add(amount).unwrap_or_revert();
        }

        self.erc1155_emit(&ERC1155Event::TransferBatch {
            operator: operator,
            from: from,
            to: to,
            ids: ids.clone(),
            amounts: amounts.clone(),
        });
        self._after_token_transfer(
            operator,
            from,
            to,
            ids.clone(),
            amounts.clone(),
            _data.clone(),
        );
        self._do_safe_batch_transfer_acceptance_check(operator, from, to, ids, amounts, _data);
    }

    fn _mint(&mut self, from: Key, to: Key, id: U256, amount: U256, _data: Bytes) {
        if !(to != self.get_caller()) {
            runtime::revert(ApiError::from(Error::MintToZeroAddress));
        }
        let operator: Key = self.get_caller();
        let ids: Vec<U256> = self._asSingletonArray(id);
        let amounts: Vec<U256> = self._asSingletonArray(amount);
        self._before_token_transfer(
            operator,
            from,
            to,
            ids.clone(),
            amounts.clone(),
            _data.clone(),
        );
        let prev_amount: U256 = Balances::instance().get(&id, &to);
        let updated_amount: U256 = prev_amount.checked_add(amount).unwrap_or_revert();

        self.erc1155_emit(&ERC1155Event::TransferSingle {
            operator: operator,
            from: data::ZERO_ADDRESS(),
            to: to,
            id: id,
            amount: amount,
        });
        self._after_token_transfer(
            operator,
            data::ZERO_ADDRESS(),
            to,
            ids,
            amounts,
            _data.clone(),
        );
        self._do_safe_transfer_acceptance_check(
            operator,
            data::ZERO_ADDRESS(),
            to,
            id,
            amount,
            _data,
        )
    }
    fn _mint_batch(&mut self, to: Key, ids: Vec<U256>, amounts: Vec<U256>, _data: Bytes) {
        if !(to != self.get_caller()) {
            runtime::revert(ApiError::from(Error::MintToZeroAddress));
        }
        if !(ids.len() == amounts.len()) {
            runtime::revert(ApiError::from(Error::MismatchIdsAndLength));
        }
        let operator: Key = self.get_caller();
        self._before_token_transfer(
            operator,
            data::ZERO_ADDRESS(),
            to,
            ids.clone(),
            amounts.clone(),
            _data.clone(),
        );
        for i in 0..ids.len() {
            let prev_amount: U256 = Balances::instance().get(&ids[i], &to);
            let updated_amount: U256 = prev_amount.checked_add(amounts[i]).unwrap_or_revert();
        }
        self.erc1155_emit(&ERC1155Event::TransferBatch {
            operator: operator,
            from: data::ZERO_ADDRESS(),
            to: to,
            ids: ids.clone(),
            amounts: amounts.clone(),
        });
        self._after_token_transfer(
            operator,
            data::ZERO_ADDRESS(),
            to,
            ids.clone(),
            amounts.clone(),
            _data.clone(),
        );
        self._do_safe_batch_transfer_acceptance_check(
            operator,
            data::ZERO_ADDRESS(),
            to,
            ids,
            amounts,
            _data,
        );
    }
    fn _burn(&mut self, from: Key, id: U256, amount: U256) {
        if !(from != self.get_caller()) {
            runtime::revert(ApiError::from(Error::BurnFromZeroAddress));
        }
        let operator: Key = self.get_caller();
        let ids: Vec<U256> = self._asSingletonArray(id);
        let amounts: Vec<U256> = self._asSingletonArray(amount);
        self._before_token_transfer(
            operator,
            from,
            data::ZERO_ADDRESS(),
            ids.clone(),
            amounts.clone(),
            "".as_bytes().into(),
        );
        let from_balance: U256 = Balances::instance().get(&id, &from);
        if !(from_balance >= amount) {
            runtime::revert(ApiError::from(Error::BurnAmountExceedsBal));
        }
        let updated_amount_from: U256 = from_balance.checked_sub(amount).unwrap_or_revert();
        Balances::instance().set(&id, &from, updated_amount_from);
        self.erc1155_emit(&ERC1155Event::TransferSingle {
            operator: operator,
            from: from,
            to: data::ZERO_ADDRESS(),
            id: id,
            amount: amount,
        });

        self._after_token_transfer(
            operator,
            from,
            data::ZERO_ADDRESS(),
            ids,
            amounts,
            "".as_bytes().into(),
        );
    }
    fn _burn_batch(&mut self, from: Key, ids: Vec<U256>, amounts: Vec<U256>) {
        if !(from != data::ZERO_ADDRESS()) {
            runtime::revert(ApiError::from(Error::BurnFromZeroAddress));
        }
        if !(ids.len() == amounts.len()) {
            runtime::revert(ApiError::from(Error::MismatchIdsAndLength));
        }
        let operator: Key = self.get_caller();
        self._before_token_transfer(
            operator,
            from,
            data::ZERO_ADDRESS(),
            ids.clone(),
            amounts.clone(),
            "".as_bytes().into(),
        );

        for i in 0..ids.len() {
            let id: U256 = ids[i];
            let amount: U256 = amounts[i];
            let from_balance: U256 = Balances::instance().get(&id, &from);
            if !(from_balance >= amount) {
                runtime::revert(ApiError::from(Error::BurnAmountExceedsBal));
            }
            let updated_amount: U256 = from_balance.checked_sub(amount).unwrap_or_revert();
            Balances::instance().set(&id, &from, updated_amount);
        }

        self.erc1155_emit(&ERC1155Event::TransferBatch {
            operator: operator,
            from: from,
            to: data::ZERO_ADDRESS(),
            ids: ids.clone(),
            amounts: amounts.clone(),
        });

        self._after_token_transfer(
            operator,
            from,
            data::ZERO_ADDRESS(),
            ids,
            amounts,
            "".as_bytes().into(),
        );
    }
    fn _set_approval_for_all(&mut self, owner: Key, operator: Key, approved: bool) {
        if !(owner != operator) {
            runtime::revert(ApiError::from(Error::SettingApprovalForSelf));
        }
        OperatorApprovals::instance().set(&owner, &operator, approved);
        self.erc1155_emit(&ERC1155Event::ApprovalForAll {
            owner: owner,
            operator: operator,
            approved: approved,
        });
    }
    fn _before_token_transfer(
        &mut self,
        operator: Key,
        from: Key,
        to: Key,
        ids: Vec<U256>,
        amounts: Vec<U256>,
        _data: Bytes,
    ) {
    }
    fn _after_token_transfer(
        &mut self,
        operator: Key,
        from: Key,
        to: Key,
        ids: Vec<U256>,
        amounts: Vec<U256>,
        _data: Bytes,
    ) {
    }
    fn _do_safe_transfer_acceptance_check(
        &mut self,
        operator: Key,
        from: Key,
        to: Key,
        id: U256,
        amount: U256,
        _data: Bytes,
    ) {
        let v = to.to_formatted_string();
        let ch = v.chars().next().unwrap();
        set_key("ch", ch.to_string());
        if eq(&ch, &'H') {
            let ret: Vec<u8> = runtime::call_contract(
                to.into_hash().unwrap_or_revert().into(),
                "on_erc1155_received",
                runtime_args! {},
            );
            runtime::revert(ApiError::from(Error::Erc115RejectedTok));
        }
    }
    fn _do_safe_batch_transfer_acceptance_check(
        &mut self,
        operator: Key,
        from: Key,
        to: Key,
        ids: Vec<U256>,
        amounts: Vec<U256>,
        _data: Bytes,
    ) {
        let v = to.to_formatted_string();
        let ch = v.chars().next().unwrap();
        set_key("ch", ch.to_string());
        if eq(&ch, &'H') {
            let ret: Vec<u8> = runtime::call_contract(
                to.into_hash().unwrap_or_revert().into(),
                "on_erc1155_received",
                runtime_args! {},
            );
            runtime::revert(ApiError::from(Error::Erc115RejectedTok));
        }
    }
    fn _asSingletonArray(&self, element: U256) -> Vec<U256> {
        let mut vec = Vec::new();
        vec.push(element);
        vec
    }
    fn _vec_to_str(&self, vec: Vec<U256>) -> String {

        let mut str = String::new();

        for i in 0..vec.len() {
            let elem: String = vec[i].to_string();
            str.push_str(&elem);
        }
        str
    }

    fn erc1155_emit(&mut self, erc1155_event: &ERC1155Event) {
        let mut events = Vec::new();
        let package = data::get_package_hash();
        match erc1155_event {
            ERC1155Event::TransferBatch {
                operator,
                from,
                to,
                ids,
                amounts,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", erc1155_event.type_name());
                event.insert("operator", operator.to_string());
                event.insert("from", from.to_string());
                event.insert("to", to.to_string());
                event.insert("ids", self._vec_to_str(ids.to_vec()));
                event.insert("amounts", self._vec_to_str(amounts.to_vec()));
                events.push(event);
            }
            ERC1155Event::TransferSingle {
                operator,
                from,
                to,
                id,
                amount,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", erc1155_event.type_name());
                event.insert("operator", operator.to_string());
                event.insert("from", from.to_string());
                event.insert("to", to.to_string());
                event.insert("id", id.to_string());
                event.insert("amount", amount.to_string());
                events.push(event);
            }
            ERC1155Event::ApprovalForAll {
                owner,
                operator,
                approved,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", erc1155_event.type_name());
                event.insert("owner", owner.to_string());
                event.insert("operator", operator.to_string());
                event.insert("approved", approved.to_string());
                events.push(event);
            }
        };
        for event in events {
            let _: URef = storage::new_uref(event);
        }
    }
}
