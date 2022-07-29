use cosmwasm_std::{StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("InvalidCDP mcr = {mcr:?} but input = {input:?}")]
    InvalidCDP {
        input: Uint128,
        mcr: u64
    },

    #[error("ValidCDP mcr = {mcr:?} but input = {input:?}")]
    ValidCDP {
        input: Uint128,
        mcr: u64
    },

    #[error("NotRegisteredCollateral: registered = {registered:?} but input = {input:?}")]
    NotRegisteredCollateral {
        registered: String,
        input: String
    },

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
