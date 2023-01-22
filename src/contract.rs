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

    STATE.save(
        deps.storage,
        &State {
            owner,
            max_bid: None,
        },
    )?;

    Ok(Response::new())
}

pub mod exec {
    use crate::contract::DENOM;
    use crate::error::ContractError;
    use cosmwasm_std::{DepsMut, MessageInfo, Response, StdError, StdResult, Uint128};

    use crate::state::{BIDS, STATE};

    pub fn bid(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
        let incoming_bid = info
            .funds
            .iter()
            .find(|x| x.denom == DENOM)
            .ok_or_else(|| StdError::generic_err(format!("No {} tokens sent", &DENOM)))?
            .amount;
        let current_bid = BIDS
            .may_load(deps.storage, &info.sender)?
            .unwrap_or_default();
        let total_bid = incoming_bid + current_bid;
        let mut state = STATE.load(deps.storage)?;

        if let Some(max_bid) = state.max_bid {
            if total_bid <= max_bid.1 {
                return Err(ContractError::BidTooSmall {});
            }
        }

        state.max_bid = Some((info.sender.clone(), total_bid));
        STATE.save(deps.storage, &state)?;
        BIDS.update(
            deps.storage,
            &info.sender,
            |balance: Option<Uint128>| -> StdResult<_> {
                Ok(balance.unwrap_or_default() + incoming_bid)
            },
        )?;

        Ok(Response::default()
            .add_attribute("action", "bid")
            .add_attribute("sender", info.sender.as_str())
            .add_attribute("total_bid", total_bid))
    }
}

pub mod query {
    use crate::msg::{BidResp, HighestResp};
    use crate::state::{BIDS, STATE};
    use cosmwasm_std::{Deps, StdResult};

    pub fn bid(deps: Deps, address: String) -> StdResult<BidResp> {
        let address = deps.api.addr_validate(&address)?;
        let balance = BIDS.may_load(deps.storage, &address)?.unwrap_or_default();
        Ok(BidResp { balance })
    }

    pub fn highest(deps: Deps) -> StdResult<Option<HighestResp>> {
        let state = STATE.load(deps.storage)?;
        let max_bid = match state.max_bid {
            Some(max_bid) => Some(HighestResp {
                address: max_bid.0,
                amount: max_bid.1,
            }),
            None => None,
        };

        Ok(max_bid)
    }
}
