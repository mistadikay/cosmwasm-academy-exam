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

    #[error("Biding is already closed")]
    BiddingClosed {},

    #[error("Biding is not closed yet")]
    BiddingNotClosed {},

    #[error("There is no bid")]
    BidMissing {},
}
