use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct State {
    pub closed: bool,
    pub owner: Addr,
    pub max_bid: Option<(Addr, Uint128)>,
}

pub const STATE: Item<State> = Item::new("state");
pub const BIDS: Map<&Addr, Uint128> = Map::new("bids");
