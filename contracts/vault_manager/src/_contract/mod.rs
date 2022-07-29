#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:stnd-vault-manager";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


pub mod instantiate;

pub mod execute;

pub mod query;

//#[cfg(test)]
//pub mod test;
