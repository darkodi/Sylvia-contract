use cosmwasm_std::{Response, StdResult};
use sylvia::{contract, entry_points};
use sylvia::types::InstantiateCtx; //  context type

pub struct CounterContract;

// macro which dispatch received messages to the handler
/*The #[entry_point] macro in Sylvia is like a translator, 
enabling the complex Rust types used in smart contract functions to be understood and handled by the simpler Wasm runtime.  */
// it goes before #[contract]!
#[entry_points] 
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
