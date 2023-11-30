use cosmwasm_std::{Response, StdResult};
use cw_storage_plus::Item;
use sylvia::{contract, entry_points};
use sylvia::types::{InstantiateCtx, QueryCtx, ExecCtx};

use crate::responses::CountResponse; //  context types

pub struct CounterContract {
    // Item<_> - this is just an accessor that allows to read a state stored on the blockchain via the key "count" in our case
    pub(crate) count: Item<'static, u32>,
}

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
        Self {
            count: Item::new("count"),
        }
    }

    #[msg(instantiate)]
    pub fn instantiate(&self, _ctx: InstantiateCtx, count: u32) -> StdResult<Response> {
        // _ctx.deps.storage - actual blockchain storage
        self.count.save(_ctx.deps.storage, &count)?; // initial value
        Ok(Response::default())
    }

    #[msg(query)]
    pub fn count(&self, ctx: QueryCtx) -> StdResult<CountResponse> {
        let count = self.count.load(ctx.deps.storage)?;
        Ok(CountResponse { count })
    }

    #[msg(exec)]
    pub fn increment_count(&self, ctx: ExecCtx, ) -> StdResult<Response> {
        self.count
            .update(ctx.deps.storage, |count| -> StdResult<u32> {
                Ok(count + 1)
            })?;
        Ok(Response::default())
    }
}
