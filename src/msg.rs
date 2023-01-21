use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Option<String>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(BidResp)]
    Bid {},
}

#[cw_serde]
pub enum ExecMsg {}

#[cw_serde]
pub struct BidResp {
    pub value: u64,
}
