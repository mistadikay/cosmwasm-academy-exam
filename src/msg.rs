use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Option<String>,
    pub commission_percent: Option<u8>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(BidResp)]
    Bid { address: String },

    #[returns(HighestResp)]
    Highest {},

    #[returns(ClosedResp)]
    Closed {},

    #[returns(WinnerResp)]
    Winner {},
}

#[cw_serde]
pub enum ExecMsg {
    Bid {},
    Close {},
    Retract { address: Option<String> },
}

#[cw_serde]
pub struct BidResp {
    pub bid: Uint128,
}

#[cw_serde]
pub struct HighestResp {
    pub address: Addr,
    pub amount: Uint128,
}

#[cw_serde]
pub struct ClosedResp {
    pub closed: bool,
}

#[cw_serde]
pub struct WinnerResp {
    pub winner: Option<HighestResp>,
}
