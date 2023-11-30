use cosmwasm_std::{StdError, Addr};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")] // variants need to be prefixed with #[error(..)] attribute
    Std(#[from] StdError), // Std implements From trait. This way we can both return standard CosmWasm errors and our own defined ones

    #[error("Cannot decrement count. Already at zero.")]
    CannotDecrementCount,
    #[error("Address '{0}' is not the owner.")]
    NotTheOwner(Addr),
}
