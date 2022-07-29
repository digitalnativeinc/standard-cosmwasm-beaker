use cosmwasm_std::{Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Initialize {
        vault_code_id_: u64,
        v1_: String,
        stablecoin_: String,
        factory_: String,
        admin_: String,
    },
    CreateVault {
        d_amount: Uint128,
    },
    SetVaultConfig {
        clt: String,
        c_decimal_: u64,
        pool_id_: u64,
        // Each rate is Percent with 5 decimals, e.g. 100% = 10000000
        mcr_: u64,
        lfr_: u64,
        sfr_: u64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetVaultConfig { clt: String },
    GetConfig {}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VaultConfigResponse {
    pub c_decimal: u64,
    pub pool_id: u64,
    pub mcr: u64,
    pub lfr: u64,
    pub sfr: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub v1: String,
    pub stablecoin: String,
    pub admin: String,
    pub vault_code_id: u64
}