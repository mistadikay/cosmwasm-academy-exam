use cosmwasm_std::Addr;
use cw_multi_test::App;

use super::contract::BiddingContract;

#[test]
fn sample_test() {
    let owner = Addr::unchecked("owner");
    let mut app = App::default();
    let code_id = BiddingContract::store_code(&mut app);

    BiddingContract::instantiate(&mut app, code_id, &owner, "Bidding contract", None, None)
        .unwrap();

    assert_eq!(owner.to_string(), "owner")
}
