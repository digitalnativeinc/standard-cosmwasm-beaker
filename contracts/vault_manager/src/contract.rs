use crate::msg::{ConfigResponse, VaultConfigResponse};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo, QueryRequest, Reply,
    ReplyOn, Response, StdResult, SubMsg, Uint128, WasmMsg,
};
use cw2::set_contract_version;
use cw_utils::parse_reply_instantiate_data;
use osmo_bindings::{OsmosisQuery, Swap};
use primitives::functions::_is_valid_cdp;
use primitives::nft::msg::{Extension, MintMsg};
use primitives::vault::functions::query_spot_price;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG, RESERVE, VAULTCONFIG};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:stnd-vault-manager";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<OsmosisQuery>,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let config = Config {
        count: 0u64,
        v1: info.sender.to_string(),
        stablecoin: info.sender.to_string(),
        factory: info.sender.to_string(),
        admin: info.sender.to_string(),
        vault_code_id: 0,
        initialized: false,
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("admin", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<OsmosisQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Initialize {
            v1_,
            stablecoin_,
            factory_,
            admin_,
            vault_code_id_,
        } => try_initialize(
            deps,
            info,
            v1_,
            stablecoin_,
            factory_,
            admin_,
            vault_code_id_,
        ),
        ExecuteMsg::CreateVault { d_amount } => try_create_vault(deps, env, info, d_amount),
        ExecuteMsg::SetVaultConfig {
            clt,
            c_decimal_,
            pool_id_,
            mcr_,
            lfr_,
            sfr_,
        } => try_set_vault_config(deps, env, info, clt, c_decimal_, pool_id_, mcr_, lfr_, sfr_),
    }
}

pub fn try_initialize(
    deps: DepsMut<OsmosisQuery>,
    info: MessageInfo,
    v1_: String,
    stablecoin_: String,
    factory_: String,
    admin_: String,
    vault_code_id_: u64,
) -> Result<Response, ContractError> {
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
    env: Env,
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
    env: Env,
    info: MessageInfo,
    d_amount: Uint128,
) -> Result<Response, ContractError> {
    // dAmount in 6 decimal precision
    let config = CONFIG
        .may_load(deps.storage)?
        .ok_or(ContractError::Uninitialized {})?;
    // TODO: get asset value of submitting collateral and stablecoin with decimal, set asset price with 3 decimal and get token decimal from the submitted token

    let input = &info.funds[0];
    let vault_config = VAULTCONFIG
        .may_load(deps.storage, (*input.denom).to_string())?
        .ok_or(ContractError::CollateralNotRegistered {
            denom: (*input.denom).to_string(),
        })?;
    // TODO: get asset
    let c = &info.funds[0];
    let d = Coin {
        denom: config.stablecoin,
        amount: d_amount,
    };

    // TODO: get asset price
    let spot_price = OsmosisQuery::spot_price(vault_config.pool_id, &c.denom, "g-usdc");
    let query = QueryRequest::from(spot_price);
    let c_price = deps.querier.query(&query)?;
    let spot_priced = OsmosisQuery::spot_price(vault_config.pool_id, &d.denom, "g-usdc");
    let query = QueryRequest::from(spot_priced);
    let d_price = deps.querier.query(&query)?;

    // calculate cdp
    if _is_valid_cdp(
        c_price,
        d_price,
        input.amount,
        d_amount,
        vault_config.c_decimal,
        vault_config.mcr,
    ) {
        let config = CONFIG.update(deps.storage, |mut config| -> Result<_, ContractError> {
            config.count += 1;
            Ok(config)
        })?;

        let reserve = crate::state::Reserve {
            vault_id: config.count,
            amount: d_amount,
            to: info.sender.to_string(),
        };

        RESERVE.save(deps.storage, &reserve)?;

        return Ok(Response::new()
            .add_attribute("method", "try_create_vault")
            .add_submessage(SubMsg {
                id: 1,
                gas_limit: None,
                msg: CosmosMsg::Wasm(WasmMsg::Instantiate {
                    code_id: config.vault_code_id,
                    funds: vec![input.clone()],
                    admin: Some(env.contract.address.to_string()),
                    label: "vault".to_string(),
                    msg: to_binary(&primitives::vault::msg::InstantiateMsg {
                        vault_id: config.count,
                        manager: env.contract.address.to_string(),
                        collateral: input.clone().denom,
                        debt: config.stablecoin,
                        v1: config.v1,
                        borrow: d_amount,
                        created_at: env.block.time.seconds(),
                    })?,
                }),
                reply_on: ReplyOn::Success,
            }));
    } else {
        return Err(ContractError::InvalidCDP {});
    }
}

/// This just stores the result for future query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(
    deps: DepsMut<OsmosisQuery>,
    _env: Env,
    msg: Reply,
) -> Result<Response, ContractError> {
    let result = match parse_reply_instantiate_data(msg) {
        Ok(res) => {
            let vault_contract = res.contract_address;
            let config = CONFIG.load(deps.storage)?;
            let reserve = RESERVE.load(deps.storage)?;
            Ok(Response::new()
                .add_attributes(vec![("vault_contract_addr", vault_contract)])
                .add_messages(vec![
                    // Mint V1
                    CosmosMsg::Wasm(WasmMsg::Execute {
                        contract_addr: config.v1.to_string(),
                        funds: vec![],
                        msg: to_binary(&primitives::nft::msg::ExecuteMsg::Mint(MintMsg::<
                            Extension,
                        > {
                            token_id: reserve.vault_id.to_string(),
                            owner: reserve.to.clone(),
                            token_uri: None,
                            extension: None,
                        }))?,
                    }),
                    // Mint Stablecoin
                    CosmosMsg::Wasm(WasmMsg::Execute {
                        contract_addr: config.stablecoin,
                        funds: vec![],
                        msg: to_binary(&primitives::token::msg::ExecuteMsg::Mint {
                            recipient: reserve.to,
                            amount: reserve.amount,
                        })?,
                    }),
                ]))
        }
        Err(e) => Err(ContractError::CustomError {
            val: (e.to_string()),
        }),
    };
    // reset reserve
    let reserve = crate::state::Reserve {
        vault_id: 0,
        amount: Uint128::zero(),
        to: "".to_string(),
    };
    RESERVE.save(deps.storage, &reserve)?;
    return result;
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetVaultConfig { clt } => to_binary(&query_vault_config(deps, clt)?),
        QueryMsg::GetConfig {} => to_binary(&query_config(deps)?)
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.may_load(deps.storage)?.unwrap_or_default();

    let resp = ConfigResponse {
        v1: config.v1,
        stablecoin: config.stablecoin,
        admin: config.admin,
        vault_code_id: config.vault_code_id,
    };
    Ok(resp)
}

fn query_vault_config(deps: Deps, clt: String) -> StdResult<VaultConfigResponse> {
    let vault_config = VAULTCONFIG.may_load(deps.storage, clt)?.unwrap_or_default();

    let resp = VaultConfigResponse {
        c_decimal: vault_config.c_decimal,
        pool_id: vault_config.pool_id,
        mcr: vault_config.mcr,
        lfr: vault_config.lfr,
        sfr: vault_config.sfr,
    };
    Ok(resp)
}

//#[cfg(test)]
//pub mod test;
