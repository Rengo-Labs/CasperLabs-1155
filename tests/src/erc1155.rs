use casper_engine_test_support::{Code, Hash, SessionBuilder, TestContext, TestContextBuilder};
use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, AsymmetricType, CLTyped, PublicKey,
    RuntimeArgs, U256, U512,
};

pub mod token_cfg {
    use super::*;
    pub const NAME: &str = "ERC1155";
    pub const SYMBOL: &str = "1155";
    pub const URI: &str = "test.io/";
}

pub struct Sender(pub AccountHash);

pub struct Token {
    context: TestContext,
    pub ali: AccountHash,
    pub bob: AccountHash,
    pub joe: AccountHash,
    pub default:AccountHash
}

impl Token {
    pub fn deployed() -> Token {
        let ali = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
        let bob = PublicKey::ed25519_from_bytes([6u8; 32]).unwrap();
        let joe = PublicKey::ed25519_from_bytes([9u8; 32]).unwrap();
        let default= AccountHash::new([0; 32]);
        let mut context = TestContextBuilder::new()
            .with_public_key(ali.clone(), U512::from(500_000_000_000_000_000u64))
            .with_public_key(bob.clone(), U512::from(500_000_000_000_000_000u64))
            .build();
        let session_code = Code::from("contract.wasm");
        let session_args = runtime_args! {
            "token_name" => token_cfg::NAME,
            "token_symbol" => token_cfg::SYMBOL,
            "base_uri" => token_cfg::URI,
            "owner"=>  ali.to_account_hash()
        };
        let session = SessionBuilder::new(session_code, session_args)
            .with_address((&ali).to_account_hash())
            .with_authorization_keys(&[ali.to_account_hash()])
            .build();
        context.run(session);
        Token {
            context,
            ali: ali.to_account_hash(),
            bob: bob.to_account_hash(),
            joe: joe.to_account_hash(),
            default:default
        }
    }

    fn contract_hash(&self) -> Hash {
        self.context
            .query(self.ali, &[format!("{}_hash", token_cfg::NAME)])
            .unwrap_or_else(|_| panic!("{} contract not found", token_cfg::NAME))
            .into_t()
            .unwrap_or_else(|_| panic!("{} has wrong type", token_cfg::NAME))
    }

    fn query_contract<T: CLTyped + FromBytes>(&self, name: &str) -> Option<T> {
        match self
            .context
            .query(self.ali, &[token_cfg::NAME.to_string(), name.to_string()])
        {
            Err(_) => None,
            Ok(maybe_value) => {
                let value = maybe_value
                    .into_t()
                    .unwrap_or_else(|_| panic!("{} is not expected type.", name));
                Some(value)
            }
        }
    }

    fn call(&mut self, sender: Sender, method: &str, args: RuntimeArgs) {
        let Sender(address) = sender;
        let code = Code::Hash(self.contract_hash(), method.to_string());
        let session = SessionBuilder::new(code, args)
            .with_address(address)
            .with_authorization_keys(&[address])
            .build();
        self.context.run(session);
    }

    pub fn name(&self) -> String {
        self.query_contract("name").unwrap()
    }

    pub fn symbol(&self) -> String {
        self.query_contract("symbol").unwrap()
    }
    pub fn total_supply(&self) -> String {
        self.query_contract("total_supply").unwrap()
    }

    pub fn balance_of(&self, account: AccountHash,id:U256) -> U256 {
        let key = format!("balances_{}_{}", account,id);
        self.query_contract(&key).unwrap_or_default()
    }
    pub fn is_minter(&self, account: AccountHash) -> bool {
        let key = format!("minter_{}", &account);
        self.query_contract(&key).unwrap_or_default()
    }
    pub fn owner(&self) -> AccountHash {
        self.query_contract("contract_owner").unwrap_or_default()
    }
    pub fn approval(&self, id: U256) -> AccountHash {
        let key = format!("approval_{}", id);
        self.query_contract(&key).unwrap_or_default()
    }
    pub fn token_uri(&self, id: U256) -> String {
        let base:String= self.query_contract("base_uri").unwrap_or_default();
        format!("{}{}", base, id)
        
    }
    pub fn approval_all(&self, owner: AccountHash, spender: AccountHash) -> bool {
        let key = format!("approval_all_{}_{}", owner, spender);
        self.query_contract(&key).unwrap_or_default()
    }

  
    pub fn mint(&mut self, recipient: AccountHash, sender: Sender, id: U256,amount:U256) {
        self.call(
            sender,
            "mint",
            runtime_args! {
                "to"=>recipient,
                "id" => id,                
                "amount"=>amount
            },
        );
    }
    pub fn batchmint(&mut self, recipient: AccountHash, sender: Sender, ids: Vec<U256>,amounts:Vec<U256>) {
        self.call(
            sender,
            "batch_mint",
            runtime_args! {
                "to"=>recipient,
                "ids" => ids,                
                "values"=>amounts
            },
        );
    }
    pub fn approve_all(&mut self, spender: AccountHash, sender: Sender) {
        self.call(
            sender,
            "approve_all",
            runtime_args! {
                "spender" => spender,

            },
        );
    }
   
    pub fn safe_transfer_from(
        &mut self,
        owner: AccountHash,
        recipient: AccountHash,
        id: U256,
        amount:U256,
        sender: Sender,
    ) {
        self.call(
            sender,
            "safe_transfer_from",
            runtime_args! {
                "owner" => owner,
                "recipient" => recipient,
                "id" => id,
                "amount"=>amount
            },
        );
    }
    pub fn batch_transfer_from(
        &mut self,
        owner: AccountHash,
        recipient: AccountHash,
        ids: Vec<U256>,
        values:Vec<U256>,
        sender: Sender,
    ) {
        self.call(
            sender,
            "transfer_from",
            runtime_args! {
                "owner" => owner,
                "recipient" => recipient,
                "ids" => ids,
                "values" => values
            },
        );
    }
}
