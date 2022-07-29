use cosmwasm_std::{Uint128, Addr, Coin};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub vault_id: u64,
    pub manager: String,
    pub collateral: String,
    pub debt: String,
    pub v1: String,
    pub borrow: Uint128,
    pub created_at: u64
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // vault custom methods
    Liquidate {},
    WithdrawCollateral { amount: Uint128 },
    DepositCollateral {},
    BorrowMore { amount: Uint128 },
    Paydebt { amount: Uint128 },
    CloseVault { }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetState {},
    GetBalances {}
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StateResponse {
    pub vault_id: u64,
    pub manager: String,
    pub collateral: String,
    pub debt: String,
    pub v1: String,
    pub borrow: Uint128,
    pub last_updated: u64,
    pub sfr: u64
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VaultBalanceResponse {
    pub c: Coin,
    pub d: Coin
}

