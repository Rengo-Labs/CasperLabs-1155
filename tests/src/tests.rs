use crate::erc1155::{token_cfg, Sender, Token};
use casper_types::{U256, U512};
#[test]
fn test_erc1155_deploy() {
    let t = Token::deployed();
    assert_eq!(t.name(), token_cfg::NAME);
    assert_eq!(t.symbol(), token_cfg::SYMBOL);

 
    
}

#[test]
fn test_erc1155_mint() {
    let mut t = Token::deployed();
    let id=83.into();
    let id1=34.into();
    let id2=29.into();
    let mut ids = vec![id, id1, id2];
    let mut quantities=vec![200.into(),200.into(),200.into()];
    t.mint(t.bob, Sender(t.ali), id,100.into());

    //t.transfer_from(t.ali,t.bob, id, Sender(t.ali));
    assert_eq!((t.balance_of(t.bob,id)),100.into());
    t.batchmint(t.bob,Sender(t.ali),quantities,ids);
    assert_eq!((t.balance_of(t.bob,id)),300.into());
    assert_eq!((t.balance_of(t.bob,id1)),300.into());
}
fn test_batch_transfer(){
    let mut t = Token::deployed();
    let id=83.into();
    let id1=34.into();
    let id2=29.into();
    let mut ids = vec![id, id1, id2];
    let mut quantities=vec![200.into(),200.into(),200.into()];
    t.batchmint(t.bob,Sender(t.ali),quantities,ids);
}