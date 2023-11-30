use cosmwasm_std::{Response, StdResult, Storage, Addr};
use cw_storage_plus::Item;
use sylvia::contract;
#[cfg(not(feature = "library"))]
use sylvia::entry_points;
use sylvia::types::{InstantiateCtx, QueryCtx, ExecCtx};

use crate::error::ContractError;
use crate::responses::CountResponse; //  context types

pub struct CounterContract<'a> {
    // Item<_> - this is just an accessor that allows to read a state stored on the blockchain via the key "count" in our case
    pub(crate) count: Item<'a, u32>,
    pub(crate) owner: Item<'a, Addr>,
    pub(crate) admins: Item<'a, Vec<Addr>>,
}

// macro which dispatch received messages to the handler
/*The #[entry_point] macro in Sylvia is like a translator, 
enabling the complex Rust types used in smart contract functions to be understood and handled by the simpler Wasm runtime.  */
// it goes before #[contract]!
#[cfg_attr(not(feature = "library"), entry_points)]
/*We mark the impl block with contract attribute macro. 
It will parse every method inside the impl block marked with the [msg(...)] attribute and create proper messages 
and utilities like multitest helpers for them. */
#[contract]
#[error(ContractError)] // To inform sylvia that it should be using a new type we add #[error(ContractError)] attribute to the contract macro call.
#[messages(crate::whitelist as Whitelist)] // 
impl CounterContract<'_> {
    pub const fn new() -> Self {
        Self {
            count: Item::new("count"),
            owner: Item::new("owner"), // Whitelist
            admins: Item::new("admins"), // Whitelist
        }
    }

    #[msg(instantiate)]
    pub fn instantiate(&self, ctx: InstantiateCtx, count: u32, admins: Vec<Addr>) -> StdResult<Response> {
        // _ctx.deps.storage - actual blockchain storage
        self.count.save(ctx.deps.storage, &count)?; // initial value
        self.owner.save(ctx.deps.storage, &ctx.info.sender)?;
        self.admins.save(ctx.deps.storage, &admins)?;
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
    #[msg(exec)]
    pub fn decrement_count(&self, ctx: ExecCtx) -> Result<Response, ContractError> {
        let count = self.count.load(ctx.deps.storage)?;
        if count == 0 {
            return Err(ContractError::CannotDecrementCount);
        }
        self.count.save(ctx.deps.storage, &(count - 1))?;
        Ok(Response::default())
    }

    // pub(crate) fn is_admin_or_owner(&self, storage: &mut dyn Storage, address: &Addr) -> bool {
    //     return self.is_owner(storage, address) || self.is_admin(storage, address);
    // }

    pub(crate) fn is_owner(&self, storage: &mut dyn Storage, address: &Addr) -> bool {
        // basically fail if unable to load state... be on the safe side
        let owner: Addr = self.owner.load(storage).unwrap_or(Addr::unchecked("error"));
        owner == address
    }

    // pub(crate) fn is_admin(&self, storage: &mut dyn Storage, address: &Addr) -> bool {
    //     // basically fail if unable to load state... be on the safe side
    //     let admins: Vec<Addr> = self.admins.load(storage).unwrap_or(vec![]);
    //     match admins.binary_search(address) {
    //         Ok(_) => true,
    //         _ => false,
    //     }
    // }
}
