pub mod query {
    use crate::msg::BidResp;
    use cosmwasm_std::{Deps, StdResult};

    pub fn bid(_deps: Deps) -> StdResult<BidResp> {
        Ok(BidResp { value: 0 })
    }
}
