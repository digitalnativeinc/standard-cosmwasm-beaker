use cosmwasm_std::{CosmosMsg, Uint128, WasmMsg, Empty};
use primitives::functions::_is_valid_cdp;

use super::*;
use crate::state::{VAULTCONFIG, CONFIG};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<OsmosisQuery>,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Initialize {  v1_, stablecoin_, factory_, admin_, vault_code_id_ } => try_initialize(deps, info, v1_, stablecoin_, factory_, admin_, vault_code_id_),
        ExecuteMsg::CreateVault { d_amount } => try_create_vault(deps, info, d_amount),
        ExecuteMsg::SetVaultConfig {
            clt,
            c_decimal_,
            pool_id_,
            mcr_,
            lfr_,
            sfr_,
        } => try_set_vault_config(deps, info, clt, c_decimal_, pool_id_, mcr_, lfr_, sfr_),
    }
}

pub fn try_initialize(deps: DepsMut<OsmosisQuery>, info: MessageInfo, v1_: String, stablecoin_: String, factory_: String, admin_: String, vault_code_id_: u64) -> Result<Response, ContractError> {
    let config = CONFIG
        .may_load(deps.storage)?
        .ok_or(ContractError::Uninitialized {})?;

    
    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    CONFIG.update(deps.storage, |config_opt| -> Result<_, ContractError> {
        let mut config = config_opt;
        config.v1 = v1_.clone();
        config.stablecoin = stablecoin_.clone();
        config.factory = factory_.clone();
        config.admin = admin_.clone();
        config.vault_code_id = vault_code_id_.clone();
        config.initialized = true;
        Ok(config)

    })?;

    Ok(Response::new()
        .add_attribute("method", "try_initialize")
        .add_attribute("vault_code_id_", vault_code_id_.to_string())
        .add_attribute("v1", v1_)
        .add_attribute("stablecoin", stablecoin_)
        .add_attribute("factory", factory_)
        
        .add_attribute("admin", admin_)
        .add_attribute("initialized", true.to_string()))
}

pub fn try_set_vault_config(
    deps: DepsMut<OsmosisQuery>,
    info: MessageInfo,
    clt: String,
    c_decimal_: u64,
    pool_id_: u64,
    mcr_: u64,
    lfr_: u64,
    sfr_: u64,
) -> Result<Response, ContractError> {
    let config = CONFIG
        .may_load(deps.storage)?
        .ok_or(ContractError::Uninitialized {})?;

    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    // TODO: check whether the pool includes stablecoin

    // Add config for the collateral
    VAULTCONFIG.update(
        deps.storage,
        clt.clone(),
        |config_opt| -> Result<_, ContractError> {
            let mut config = config_opt.unwrap_or_default();
            config.c_decimal = c_decimal_;
            config.mcr = mcr_;
            config.lfr = lfr_;
            config.sfr = sfr_;
            config.pool_id = pool_id_;
            Ok(config)
        },
    )?;
    // TODO: send event for initializing a vault config
    Ok(Response::new()
        .add_attribute("method", "try_initialize_config")
        .add_attribute("clt", clt)
        .add_attribute("mcr", mcr_.to_string())
        .add_attribute("lfr", lfr_.to_string())
        .add_attribute("sfr", sfr_.to_string())
        .add_attribute("pool_id", pool_id_.to_string()))
}

pub fn try_create_vault(
    deps: DepsMut<OsmosisQuery>,
    info: MessageInfo,
    d_amount: Uint128,
) -> Result<Response, ContractError> {
    // dAmount in 9 decimal precision
    // TODO: get asset value of submitting collateral and stablecoin with decimal, set asset price with 3 decimal and get token decimal from the submitted token
    // TODO: get asset price
    let c_price = Uint128::from(1u64);
    let d_price = Uint128::from(1u64);
    let input = &info.funds[0];
    let vault_config = VAULTCONFIG
        .may_load(deps.storage, (*input.denom).to_string())?
        .ok_or(ContractError::CollateralNotRegistered { denom: (*input.denom).to_string() })?;
    // TODO: get asset
    // calculate cdp
    let messages: Vec<CosmosMsg> = match _is_valid_cdp(
        c_price,
        d_price,
        input.amount,
        d_amount,
        vault_config.c_decimal,
        vault_config.mcr,
    ) {
        true => {
            let config = CONFIG
                .may_load(deps.storage)?
                .ok_or(ContractError::Uninitialized {})?;
            Ok(vec![
                // Call factory to mint vault
                CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: config.v1,
                    funds: vec![input.clone()],
                    msg: to_binary(&primitives::vault_factory::msg::ExecuteMsg::CreateVault {
                        c_denom: (*input.denom).to_string(),
                        c_amount: input.amount,
                        owner: info.sender.to_string()
                    })?
                }),
                // Mint Stablecoin
                CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: config.stablecoin,
                    funds: vec![],
                    msg: to_binary(&primitives::token::msg::ExecuteMsg::Mint {
                        recipient: info.sender.to_string(),
                        amount: d_amount,
                    })?,
                }),
            ])
        }
        false => Err(ContractError::InvalidCDP {})
    }?;
    Ok(Response::new()
        .add_attribute("method", "try_create_vault")
        .add_messages(messages))
}
