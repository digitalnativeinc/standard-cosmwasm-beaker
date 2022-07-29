#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary, QueryRequest, WasmQuery};
use cw2::set_contract_version;
use osmo_bindings::OsmosisQuery;
use primitives::vault_manager::msg::VaultConfigResponse;

use crate::error::ContractError;
use crate::msg::{ ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};


// version info for migration info
const CONTRACT_NAME: &str = "crates.io:stnd-vault";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<OsmosisQuery>,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    let vault_config: VaultConfigResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: msg.manager.clone(),
            msg: to_binary(&primitives::vault_manager::msg::QueryMsg::GetVaultConfig {
                clt: msg.collateral.clone(),
            })?,
        }))?;


    let state = State {
        vault_id: msg.vault_id,
        manager: msg.manager,
        debt: msg.debt,
        collateral: msg.collateral,
        borrow: msg.borrow,
        last_updated: msg.created_at,
        ex_sfr: vault_config.sfr,
        v1: msg.v1
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // Set NFT lock
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("vault_id", msg.vault_id.to_string()))
}

pub mod execute;

pub mod query;