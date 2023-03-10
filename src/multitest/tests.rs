use crate::error::ContractError;
use crate::msg::{BidResp, HighestResp};
use cosmwasm_std::{coins, Addr, Uint128};
use cw_multi_test::App;

use super::contract::BiddingContract;

const ATOM: &str = "ATOM";

#[test]
fn query_bid() {
    let owner = Addr::unchecked("owner");
    let mut app = App::default();
    let code_id = BiddingContract::store_code(&mut app);

    let contract = BiddingContract::instantiate(
        &mut app,
        code_id,
        &owner,
        "Bidding contract",
        None,
        None,
        None,
    )
    .unwrap();

    let resp = contract.query_bid(&app, owner.to_string()).unwrap();

    assert_eq!(
        resp,
        BidResp {
            bid: Uint128::new(0)
        }
    )
}

#[test]
fn query_closed() {
    let owner = Addr::unchecked("owner");
    let mut app = App::default();
    let code_id = BiddingContract::store_code(&mut app);

    let contract = BiddingContract::instantiate(
        &mut app,
        code_id,
        &owner,
        "Bidding contract",
        None,
        None,
        None,
    )
    .unwrap();

    assert_eq!(contract.query_closed(&app).unwrap().closed, false);

    contract.close(&mut app, &owner).unwrap();

    assert_eq!(contract.query_closed(&app).unwrap().closed, true);
}

#[test]
fn query_highest_bid() {
    let owner = Addr::unchecked("owner");
    let sender = Addr::unchecked("sender");
    let sender2 = Addr::unchecked("sender2");
    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender, coins(100, ATOM))
            .unwrap();
        router
            .bank
            .init_balance(storage, &sender2, coins(100, ATOM))
            .unwrap();
    });
    let code_id = BiddingContract::store_code(&mut app);

    let contract = BiddingContract::instantiate(
        &mut app,
        code_id,
        &owner,
        "Bidding contract",
        None,
        None,
        None,
    )
    .unwrap();

    let resp = contract.query_highest_bid(&app).unwrap();
    assert_eq!(resp, None);

    contract.bid(&mut app, &sender, &coins(10, ATOM)).unwrap();
    let resp = contract.query_highest_bid(&app).unwrap();
    assert_eq!(
        resp,
        Some(HighestResp {
            address: sender.clone(),
            amount: Uint128::new(10)
        })
    );

    contract.bid(&mut app, &sender2, &coins(11, ATOM)).unwrap();
    let resp = contract.query_highest_bid(&app).unwrap();
    assert_eq!(
        resp,
        Some(HighestResp {
            address: sender2.clone(),
            amount: Uint128::new(11)
        })
    );
}

#[test]
fn query_winner() {
    let owner = Addr::unchecked("owner");
    let sender = Addr::unchecked("sender");
    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender, coins(100, ATOM))
            .unwrap();
    });
    let code_id = BiddingContract::store_code(&mut app);

    let contract = BiddingContract::instantiate(
        &mut app,
        code_id,
        &owner,
        "Bidding contract",
        None,
        None,
        None,
    )
    .unwrap();

    let resp = contract.query_winner(&app).unwrap().winner;
    assert_eq!(resp, None);

    contract.bid(&mut app, &sender, &coins(10, ATOM)).unwrap();
    let resp = contract.query_winner(&app).unwrap().winner;
    assert_eq!(resp, None);

    contract.close(&mut app, &owner).unwrap();

    let resp = contract.query_winner(&app).unwrap().winner;
    assert_eq!(
        resp,
        Some(HighestResp {
            address: sender.clone(),
            amount: Uint128::new(10)
        })
    );
}

#[test]
fn bid_with_funds() {
    let owner = Addr::unchecked("owner");
    let sender = Addr::unchecked("sender");

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &owner, coins(10, ATOM))
            .unwrap();
        router
            .bank
            .init_balance(storage, &sender, coins(10, ATOM))
            .unwrap();
    });
    let code_id = BiddingContract::store_code(&mut app);

    let contract = BiddingContract::instantiate(
        &mut app,
        code_id,
        &owner,
        "Bidding contract",
        None,
        None,
        None,
    )
    .unwrap();

    contract.bid(&mut app, &sender, &coins(5, ATOM)).unwrap();

    let resp = contract.query_bid(&app, sender.to_string()).unwrap();
    assert_eq!(
        resp,
        BidResp {
            bid: Uint128::new(5)
        }
    );

    let err = contract.bid(&mut app, &owner, &coins(5, ATOM)).unwrap_err();
    assert_eq!(err, ContractError::Unauthorized {});
}

#[test]
fn bid_too_small() {
    let owner = Addr::unchecked("owner");
    let sender = Addr::unchecked("sender");
    let sender2 = Addr::unchecked("sender2");

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender, coins(100, ATOM))
            .unwrap();
        router
            .bank
            .init_balance(storage, &sender2, coins(100, ATOM))
            .unwrap();
    });
    let code_id = BiddingContract::store_code(&mut app);

    let contract = BiddingContract::instantiate(
        &mut app,
        code_id,
        &owner,
        "Bidding contract",
        None,
        None,
        None,
    )
    .unwrap();

    contract.bid(&mut app, &sender, &coins(10, ATOM)).unwrap();
    let err = contract
        .bid(&mut app, &sender2, &coins(5, ATOM))
        .unwrap_err();
    assert_eq!(err, ContractError::BidTooSmall {});

    contract.bid(&mut app, &sender2, &coins(11, ATOM)).unwrap();
    let resp = contract.query_bid(&app, sender2.to_string()).unwrap();
    assert_eq!(
        resp,
        BidResp {
            bid: Uint128::new(11)
        }
    );

    contract.bid(&mut app, &sender, &coins(2, ATOM)).unwrap();
    let resp = contract.query_bid(&app, sender.to_string()).unwrap();
    assert_eq!(
        resp,
        BidResp {
            bid: Uint128::new(12)
        }
    );
}

#[test]
fn close() {
    let owner = Addr::unchecked("owner");
    let sender1 = Addr::unchecked("sender1");
    let sender2 = Addr::unchecked("sender2");

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender1, coins(20, ATOM))
            .unwrap();
        router
            .bank
            .init_balance(storage, &sender2, coins(20, ATOM))
            .unwrap();
    });
    let code_id = BiddingContract::store_code(&mut app);

    let contract = BiddingContract::instantiate(
        &mut app,
        code_id,
        &owner,
        "Bidding contract",
        None,
        None,
        None,
    )
    .unwrap();

    contract.bid(&mut app, &sender1, &coins(5, ATOM)).unwrap();
    contract.bid(&mut app, &sender2, &coins(10, ATOM)).unwrap();
    contract.bid(&mut app, &sender1, &coins(6, ATOM)).unwrap();

    let err = contract.close(&mut app, &sender2).unwrap_err();
    assert_eq!(err, ContractError::Unauthorized {});

    contract.close(&mut app, &owner).unwrap();

    let err = contract
        .bid(&mut app, &sender1, &coins(1, ATOM))
        .unwrap_err();
    assert_eq!(err, ContractError::BiddingClosed {});

    let err = contract.close(&mut app, &owner).unwrap_err();
    assert_eq!(err, ContractError::BiddingClosed {});

    let balance1 = app
        .wrap()
        .query_balance(sender1.clone(), ATOM)
        .unwrap()
        .amount
        .u128();
    assert_eq!(balance1, 20);
    let balance2 = app
        .wrap()
        .query_balance(sender2, ATOM)
        .unwrap()
        .amount
        .u128();
    assert_eq!(balance2, 10);

    let err = contract.retract(&mut app, &sender1, None).unwrap_err();
    assert_eq!(err, ContractError::BidMissing {});
}

#[test]
fn retract() {
    let owner = Addr::unchecked("owner");
    let sender1 = Addr::unchecked("sender1");
    let sender2 = Addr::unchecked("sender2");
    let sender3 = Addr::unchecked("sender3");
    let receiver = Addr::unchecked("receiver");
    let initial_balance = coins(30, ATOM);

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender1, initial_balance.clone())
            .unwrap();
        router
            .bank
            .init_balance(storage, &sender2, initial_balance.clone())
            .unwrap();
        router
            .bank
            .init_balance(storage, &sender3, initial_balance.clone())
            .unwrap();
    });
    let code_id = BiddingContract::store_code(&mut app);

    let contract = BiddingContract::instantiate(
        &mut app,
        code_id,
        &owner,
        "Bidding contract",
        None,
        None,
        Some(10),
    )
    .unwrap();

    // total commission: 10+20+30 * 10% = 6
    contract.bid(&mut app, &sender1, &coins(10, ATOM)).unwrap();
    contract.bid(&mut app, &sender2, &coins(20, ATOM)).unwrap();
    contract.bid(&mut app, &sender3, &coins(30, ATOM)).unwrap();

    let err = contract.retract(&mut app, &sender1, None).unwrap_err();
    assert_eq!(err, ContractError::BiddingNotClosed {});

    // 30 initial balance - (3 commission from 30 bid) + 6 total commission
    contract.close(&mut app, &owner).unwrap();
    assert_eq!(
        app.wrap()
            .query_balance(sender3.clone(), ATOM)
            .unwrap()
            .amount
            .u128(),
        33
    );

    // 30 initial balance - 1 commission from 10 bid
    contract.retract(&mut app, &sender1, None).unwrap();
    assert_eq!(
        app.wrap()
            .query_balance(sender1.clone(), ATOM)
            .unwrap()
            .amount
            .u128(),
        29
    );

    // 20 bid - 2 commission (custom receiver with 0 initial balance)
    contract
        .retract(&mut app, &sender2, Some(receiver.to_string()))
        .unwrap();
    assert_eq!(
        app.wrap()
            .query_balance(receiver.clone(), ATOM)
            .unwrap()
            .amount
            .u128(),
        18
    );

    let err = contract.retract(&mut app, &sender1, None).unwrap_err();
    assert_eq!(err, ContractError::BidMissing {});
}
