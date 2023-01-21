use crate::msg::InstantiateMsg;
use crate::state::{State, STATE};
use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn instantiate(deps: DepsMut, info: MessageInfo, msg: InstantiateMsg) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let mut owner = info.sender;
    if let Some(owner_addr) = msg.owner {
        owner = deps.api.addr_validate(&owner_addr)?
    }

    STATE.save(deps.storage, &State { owner })?;

    Ok(Response::new())
}

pub mod query {
    use crate::msg::BidResp;
    use cosmwasm_std::{Deps, StdResult};

    pub fn bid(_deps: Deps) -> StdResult<BidResp> {
        Ok(BidResp { value: 0 })
    }
}
