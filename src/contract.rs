use crate::msg::InstantiateMsg;
use crate::state::{State, STATE};
use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult, Uint128};
use cw2::set_contract_version;

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const DENOM: &str = "ATOM";
const COMMISSION_DEFAULT: u8 = 5;

pub fn instantiate(deps: DepsMut, info: MessageInfo, msg: InstantiateMsg) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let mut owner = info.sender;
    if let Some(owner_addr) = msg.owner {
        owner = deps.api.addr_validate(&owner_addr)?
    }

    let commission_percent = msg.commission_percent.unwrap_or(COMMISSION_DEFAULT);
    STATE.save(
        deps.storage,
        &State {
            closed: false,
            owner,
            commission_total: Uint128::new(0),
            commission_percent,
            max_bid: None,
        },
    )?;

    Ok(Response::new())
}

pub mod exec {
    use crate::contract::DENOM;
    use crate::error::ContractError;
    use crate::error::ContractError::BidMissing;
    use cosmwasm_std::{
        coins, BankMsg, DepsMut, MessageInfo, Response, StdError, StdResult, Uint128,
    };

    use crate::state::{Bid, BIDS, STATE};

    pub fn bid(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
        let mut state = STATE.load(deps.storage)?;
        if info.sender.clone() == state.owner {
            return Err(ContractError::Unauthorized {});
        }

        if state.closed {
            return Err(ContractError::BiddingClosed {});
        }

        let incoming_bid = info
            .funds
            .iter()
            .find(|x| x.denom == DENOM)
            .ok_or_else(|| StdError::generic_err(format!("No {} tokens sent", &DENOM)))?
            .amount;
        let current_bid = BIDS
            .may_load(deps.storage, &info.sender)?
            .unwrap_or_default();
        let total_bid = incoming_bid + current_bid.retractable + current_bid.commission;

        if let Some(max_bid) = state.max_bid {
            if total_bid <= max_bid.1 {
                return Err(ContractError::BidTooSmall {});
            }
        }

        // save new max bid for future comparisons
        state.max_bid = Some((info.sender.clone(), total_bid));

        // calculate commission and retractable right away
        let commission = incoming_bid * Uint128::from(state.commission_percent) / Uint128::new(100);
        let retractable = incoming_bid - commission;
        BIDS.update(
            deps.storage,
            &info.sender,
            |bid: Option<Bid>| -> StdResult<_> {
                let bid = bid.unwrap_or_default();
                Ok(Bid {
                    commission: bid.commission + commission,
                    retractable: bid.retractable + retractable,
                })
            },
        )?;

        // future winning pot
        state.commission_total += commission;
        STATE.save(deps.storage, &state)?;

        Ok(Response::default()
            .add_attribute("action", "bid")
            .add_attribute("sender", info.sender.as_str())
            .add_attribute("total_bid", total_bid))
    }

    pub fn retract(
        deps: DepsMut,
        info: MessageInfo,
        address: Option<String>,
    ) -> Result<Response, ContractError> {
        let state = STATE.load(deps.storage)?;

        if !state.closed {
            return Err(ContractError::BiddingNotClosed {});
        }

        let mut messages = vec![];

        let current_bid = BIDS.may_load(deps.storage, &info.sender)?;
        match current_bid {
            Some(bid) => {
                let mut to_address = info.sender.clone();

                if let Some(address) = address {
                    to_address = deps.api.addr_validate(&address).unwrap_or(to_address);
                }

                messages.push(BankMsg::Send {
                    to_address: to_address.to_string(),
                    amount: coins(u128::from(bid.retractable), DENOM),
                })
            }
            None => return Err(BidMissing {}),
        }

        BIDS.remove(deps.storage, &info.sender);

        Ok(Response::new().add_messages(messages))
    }

    pub fn close(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
        let mut messages = vec![];
        let mut state = STATE.load(deps.storage)?;
        if info.sender.clone() != state.owner {
            return Err(ContractError::Unauthorized {});
        }

        if state.closed {
            return Err(ContractError::BiddingClosed {});
        }

        // if bidding commenced, send funds to the winner
        if let Some(ref max_bid) = state.max_bid {
            let winner_addr = max_bid.clone().0;
            let winner_bid = BIDS.may_load(deps.storage, &winner_addr).unwrap();

            match winner_bid {
                Some(bid) => {
                    let jackpot = bid.retractable + state.commission_total;
                    BIDS.remove(deps.storage, &winner_addr);
                    messages.push(BankMsg::Send {
                        to_address: winner_addr.to_string(),
                        amount: coins(u128::from(jackpot), DENOM),
                    })
                }
                None => return Err(BidMissing {}),
            }
        }

        state.closed = true;
        STATE.save(deps.storage, &state)?;

        Ok(Response::new().add_messages(messages))
    }
}

pub mod query {
    use crate::msg::{BidResp, ClosedResp, HighestResp, WinnerResp};
    use crate::state::{BIDS, STATE};
    use cosmwasm_std::{Deps, StdResult};

    pub fn bid(deps: Deps, address: String) -> StdResult<BidResp> {
        let address = deps.api.addr_validate(&address)?;
        let bid = BIDS.may_load(deps.storage, &address)?.unwrap_or_default();
        Ok(BidResp {
            bid: bid.retractable + bid.commission,
        })
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

    pub fn winner(deps: Deps) -> StdResult<WinnerResp> {
        let state = STATE.load(deps.storage)?;
        let mut winner = None;

        if state.closed {
            if let Some(max_bid) = state.max_bid {
                winner = Some(HighestResp {
                    address: max_bid.0,
                    amount: max_bid.1,
                });
            }
        }

        Ok(WinnerResp { winner })
    }

    pub fn closed(deps: Deps) -> StdResult<ClosedResp> {
        let state = STATE.load(deps.storage)?;

        Ok(ClosedResp {
            closed: state.closed,
        })
    }
}
