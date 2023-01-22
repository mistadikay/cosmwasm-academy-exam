use crate::msg::InstantiateMsg;
use crate::state::{State, STATE};
use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const DENOM: &str = "ATOM";

pub fn instantiate(deps: DepsMut, info: MessageInfo, msg: InstantiateMsg) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let mut owner = info.sender;
    if let Some(owner_addr) = msg.owner {
        owner = deps.api.addr_validate(&owner_addr)?
    }

    STATE.save(deps.storage, &State { owner })?;

    Ok(Response::new())
}

pub mod exec {
    use crate::contract::DENOM;
    use cosmwasm_std::{DepsMut, MessageInfo, Response, StdError, StdResult, Uint128};

    use crate::state::BIDS;

    pub fn bid(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
        let payment = info
            .funds
            .iter()
            .find(|x| x.denom == DENOM)
            .ok_or_else(|| StdError::generic_err(format!("No {} tokens sent", &DENOM)))?;

        BIDS.update(
            deps.storage,
            &info.sender,
            |balance: Option<Uint128>| -> StdResult<_> {
                Ok(balance.unwrap_or_default() + payment.amount)
            },
        )?;

        Ok(Response::default()
            .add_attribute("action", "bid")
            .add_attribute("sender", info.sender.as_str())
            .add_attribute("funds", payment.to_string()))
    }
}

pub mod query {
    use crate::msg::BidResp;
    use crate::state::BIDS;
    use cosmwasm_std::{Deps, StdResult};

    pub fn bid(deps: Deps, address: String) -> StdResult<BidResp> {
        let address = deps.api.addr_validate(&address)?;
        let balance = BIDS.may_load(deps.storage, &address)?.unwrap_or_default();
        Ok(BidResp { balance })
    }
}
