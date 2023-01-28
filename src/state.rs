use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct State {
    pub closed: bool,
    pub owner: Addr,
    pub commission_total: Uint128,
    pub commission_percent: u8,
    pub max_bid: Option<(Addr, Uint128)>,
}
pub const STATE: Item<State> = Item::new("state");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Bid {
    pub retractable: Uint128,
    pub commission: Uint128,
}
pub const BIDS: Map<&Addr, Bid> = Map::new("bids");

impl Default for Bid {
    fn default() -> Bid {
        Bid {
            retractable: Uint128::new(0),
            commission: Uint128::new(0),
        }
    }
}
