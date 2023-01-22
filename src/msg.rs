use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Option<String>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(BidResp)]
    Bid { address: String },
}

#[cw_serde]
pub enum ExecMsg {
    Bid {},
}

#[cw_serde]
pub struct BidResp {
    pub balance: Uint128,
}
