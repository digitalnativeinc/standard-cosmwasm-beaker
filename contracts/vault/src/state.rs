use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Timestamp, Uint128};
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub vault_id: u64,
    pub manager: String,
    pub collateral: String,
    pub debt: String,
    pub v1: String,
    pub borrow: Uint128,
    pub last_updated: u64,
    pub ex_sfr: u64
}

pub const STATE: Item<State> = Item::new("state");
