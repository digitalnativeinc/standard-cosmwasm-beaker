use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Uint128;
use cw_storage_plus::{Item, Map};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
pub struct VaultConfig {
    /// Collateral Decimal
    pub c_decimal: u64,
    /// Maximum Collateral Ratio
    pub mcr: u64,
    /// Liquidation Fee Ratio
    pub lfr: u64,
    /// Stability Fee Ratio (interest rate)
    pub sfr: u64,
    /// Pool Id to get price
    pub pool_id: u64
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
pub struct Config {
    pub count: u64,
    pub v1: String,
    pub stablecoin: String,
    pub factory: String,
    pub admin: String,
    pub vault_code_id: u64,
    pub initialized: bool
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
pub struct Reserve {
    pub vault_id: u64,
    pub amount: Uint128,
    pub to: String
}

pub const VAULTCONFIG: Map<String, VaultConfig> = Map::new("vault_config");
pub const CONFIG: Item<Config> = Item::new("config");
pub const RESERVE: Item<Reserve> = Item::new("reserve");