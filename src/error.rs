use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unexpected error")]
    Unexpected {},

    #[error("Unauthorized action")]
    Unauthorized {},

    #[error("Bid is not enough to beat the max bid")]
    BidTooSmall {},
}
