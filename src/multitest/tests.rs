use crate::msg::BidResp;
use cosmwasm_std::{coins, Addr, Uint128};
use cw_multi_test::App;

use super::contract::BiddingContract;

const ATOM: &str = "ATOM";

#[test]
fn query_bid() {
    let owner = Addr::unchecked("owner");
    let mut app = App::default();
    let code_id = BiddingContract::store_code(&mut app);

    let contract =
        BiddingContract::instantiate(&mut app, code_id, &owner, "Bidding contract", None, None)
            .unwrap();

    let resp = contract.query_bid(&app, owner.to_string()).unwrap();

    assert_eq!(
        resp,
        BidResp {
            balance: Uint128::new(0)
        }
    )
}

#[test]
fn bid_with_funds() {
    let owner = Addr::unchecked("owner");
    let sender = Addr::unchecked("sender");

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender, coins(10, ATOM))
            .unwrap();
    });
    let code_id = BiddingContract::store_code(&mut app);

    let contract =
        BiddingContract::instantiate(&mut app, code_id, &owner, "Bidding contract", None, None)
            .unwrap();

    contract.bid(&mut app, &sender, &coins(5, ATOM)).unwrap();

    let resp = contract.query_bid(&app, sender.to_string()).unwrap();
    assert_eq!(
        resp,
        BidResp {
            balance: Uint128::new(5)
        }
    );
}
