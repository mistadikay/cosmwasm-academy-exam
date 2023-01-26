use cosmwasm_std::{Addr, Coin, StdResult};
use cw_multi_test::{App, ContractWrapper, Executor};

use crate::error::ContractError;
use crate::msg::{BidResp, ExecMsg, HighestResp, InstantiateMsg, QueryMsg};
use crate::{execute, instantiate, query};

pub struct BiddingContract(Addr);

impl BiddingContract {
    pub fn addr(&self) -> &Addr {
        &self.0
    }

    pub fn store_code(app: &mut App) -> u64 {
        let contract = ContractWrapper::new(execute, instantiate, query);
        app.store_code(Box::new(contract))
    }

    #[track_caller]
    pub fn instantiate<'a>(
        app: &mut App,
        code_id: u64,
        sender: &Addr,
        label: &str,
        admin: impl Into<Option<&'a Addr>>,
        owner: Option<String>,
    ) -> StdResult<Self> {
        let admin = admin.into();

        app.instantiate_contract(
            code_id,
            sender.clone(),
            &InstantiateMsg { owner },
            &[],
            label,
            admin.map(Addr::to_string),
        )
        .map(BiddingContract)
        .map_err(|err| err.downcast().unwrap())
    }

    #[track_caller]
    pub fn bid(&self, app: &mut App, sender: &Addr, funds: &[Coin]) -> Result<(), ContractError> {
        app.execute_contract(sender.clone(), self.0.clone(), &ExecMsg::Bid {}, funds)
            .map_err(|err| err.downcast().unwrap())
            .map(|_| ())
    }

    #[track_caller]
    pub fn close(&self, app: &mut App, sender: &Addr) -> Result<(), ContractError> {
        app.execute_contract(sender.clone(), self.0.clone(), &ExecMsg::Close {}, &[])
            .map_err(|err| err.downcast().unwrap())
            .map(|_| ())
    }

    #[track_caller]
    pub fn query_bid(&self, app: &App, address: String) -> StdResult<BidResp> {
        app.wrap()
            .query_wasm_smart(self.0.clone(), &QueryMsg::Bid { address })
    }

    #[track_caller]
    pub fn query_highest_bid(&self, app: &App) -> StdResult<Option<HighestResp>> {
        app.wrap()
            .query_wasm_smart(self.0.clone(), &QueryMsg::Highest {})
    }
}

impl From<BiddingContract> for Addr {
    fn from(contract: BiddingContract) -> Self {
        contract.0
    }
}
