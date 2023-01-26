use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Option<String>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(BidResp)]
    Bid { address: String },

    #[returns(HighestResp)]
    Highest {},
}

#[cw_serde]
pub enum ExecMsg {
    Bid {},
    Close {},
}

#[cw_serde]
pub struct BidResp {
    pub balance: Uint128,
}

#[cw_serde]
pub struct HighestResp {
    pub address: Addr,
    pub amount: Uint128,
}
