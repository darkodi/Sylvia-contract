use cosmwasm_std::{Response, StdResult};
use sylvia::contract;
use sylvia::types::InstantiateCtx; //  context type

pub struct CounterContract;

/*We mark the impl block with contract attribute macro. 
It will parse every method inside the impl block marked with the [msg(...)] attribute and create proper messages 
and utilities like multitest helpers for them. */
#[contract]
impl CounterContract {
    pub const fn new() -> Self {
        Self
    }

    #[msg(instantiate)]
    pub fn instantiate(&self, _ctx: InstantiateCtx) -> StdResult<Response> {
        Ok(Response::default())
    }
}
