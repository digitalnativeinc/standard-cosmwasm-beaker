#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, QueryRequest, Response, StdResult,
    WasmQuery,
};
use cw2::set_contract_version;
use osmo_bindings::OsmosisQuery;
use primitives::vault_manager::msg::VaultConfigResponse;

use crate::error::ContractError;
use crate::msg::{StateResponse, VaultBalanceResponse};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};
use cosmwasm_std::{BankMsg, Coin, CosmosMsg, Uint128};
use osmosis_std::types::osmosis::{gamm::v1beta1::MsgJoinPool, tokenfactory::v1beta1::MsgBurn};
use primitives::{
    functions::{_cr, _is_valid_cdp},
    vault_manager::msg::ConfigResponse,
};

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
        v1: msg.v1,
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // Set NFT lock
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("vault_id", msg.vault_id.to_string()))
}

//pub mod execute;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<OsmosisQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Liquidate {} => try_liquidate(deps, env, info),
        ExecuteMsg::WithdrawCollateral { amount } => {
            try_withdraw_collateral(deps, env, info, amount)
        }
        ExecuteMsg::DepositCollateral {} => try_deposit_collateral(deps, env, info),
        ExecuteMsg::BorrowMore { amount } => todo!(),
        ExecuteMsg::Paydebt { amount } => try_pay_debt(deps, env, info, amount),
        ExecuteMsg::CloseVault {} => try_close_vault(deps, env, info),
    }
}

pub fn try_close_vault(
    deps: DepsMut<OsmosisQuery>,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let resp: primitives::nft::msg::OwnerOfResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: state.manager.clone(),
            msg: to_binary(&primitives::nft::msg::QueryMsg::OwnerOf {
                token_id: state.vault_id.to_string(),
                include_expired: Some(true),
            })?,
        }))?;

    if info.sender != resp.owner {
        return Err(ContractError::Unauthorized {});
    }

    let c = deps
        .querier
        .query_balance(&env.contract.address, state.collateral)?;
    let d = deps
        .querier
        .query_balance(&env.contract.address, state.debt.clone())?;

    let vault_config: VaultConfigResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: state.manager.clone(),
            msg: to_binary(&primitives::vault_manager::msg::QueryMsg::GetVaultConfig {
                clt: c.denom.clone(),
            })?,
        }))?;
    let spot_price = OsmosisQuery::spot_price(vault_config.pool_id, &c.denom, "g-usdc");
    let query = QueryRequest::from(spot_price);
    let c_price = deps.querier.query(&query)?;
    let spot_priced = OsmosisQuery::spot_price(vault_config.pool_id, &d.denom, "g-usdc");
    let query = QueryRequest::from(spot_priced);
    let d_price = deps.querier.query(&query)?;

    if !_is_valid_cdp(
        c_price,
        d_price,
        c.amount,
        d.amount,
        vault_config.c_decimal,
        vault_config.mcr,
    ) {
        return Err(ContractError::InvalidCDP {
            input: _cr(
                c_price,
                d_price,
                c.amount,
                d.amount,
                vault_config.c_decimal,
                vault_config.mcr,
            ),
            mcr: vault_config.mcr,
        });
    }

    // get vault config and config
    let config: ConfigResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: state.manager,
        msg: to_binary(&primitives::vault_manager::msg::QueryMsg::GetConfig {})?,
    }))?;

    // send back clt
    let send_back_clt: CosmosMsg = CosmosMsg::Bank(BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![c],
    });

    // process fees in sfr
    let spot_priced = OsmosisQuery::spot_price(vault_config.pool_id, &d.denom, "g-usdc");
    let query = QueryRequest::from(spot_priced);
    let d_price: Uint128 = deps.querier.query(&query)?;
    let d_value = d_price * d.amount;
    // (duration in months with 6 precision) * (sfr * assetValue/100(with 5decimals))
    let duration = ((env.block.time.seconds() - state.last_updated) * u64::pow(10, 6)) / 259200;
    let duration_v = (Uint128::from(duration) * d_value) / Uint128::from(u64::pow(10, 6));
    let fee = duration_v * Uint128::from(vault_config.sfr) / Uint128::from(10000000u64);

    let send_fee: CosmosMsg = CosmosMsg::Bank(BankMsg::Send {
        to_address: config.admin.to_string(),
        amount: vec![Coin {
            denom: d.denom.clone(),
            amount: fee,
        }],
    });

    let deduct_fee = d.amount - fee;

    let osmo_d = osmosis_std::cosmos_sdk_proto::cosmos::base::v1beta1::Coin {
        denom: d.denom.clone(),
        amount: deduct_fee.to_string(),
    };

    // burn stablecoins
    let burn_stables: CosmosMsg = MsgBurn {
        sender: config.admin,
        amount: Some(osmo_d),
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "liquidate")
        .add_message(send_back_clt)
        .add_message(send_fee)
        .add_message(burn_stables))
}

pub fn try_liquidate(
    deps: DepsMut<OsmosisQuery>,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;

    let c = deps
        .querier
        .query_balance(&env.contract.address, state.collateral)?;
    let d = deps
        .querier
        .query_balance(&env.contract.address, state.debt.clone())?;

    let vault_config: VaultConfigResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: state.manager.clone(),
            msg: to_binary(&primitives::vault_manager::msg::QueryMsg::GetVaultConfig {
                clt: c.denom.clone(),
            })?,
        }))?;
    let spot_price = OsmosisQuery::spot_price(vault_config.pool_id, &c.denom, "g-usdc");
    let query = QueryRequest::from(spot_price);
    let c_price = deps.querier.query(&query)?;
    let spot_priced = OsmosisQuery::spot_price(vault_config.pool_id, &d.denom, "g-usdc");
    let query = QueryRequest::from(spot_priced);
    let d_price = deps.querier.query(&query)?;

    if _is_valid_cdp(
        c_price,
        d_price,
        c.amount,
        d.amount,
        vault_config.c_decimal,
        vault_config.mcr,
    ) {
        return Err(ContractError::ValidCDP {
            input: _cr(
                c_price,
                d_price,
                c.amount,
                d.amount,
                vault_config.c_decimal,
                vault_config.mcr,
            ),
            mcr: vault_config.mcr,
        });
    }

    // get vault config and config
    let config: ConfigResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: state.manager,
        msg: to_binary(&primitives::vault_manager::msg::QueryMsg::GetConfig {})?,
    }))?;

    let osmo_c = osmosis_std::cosmos_sdk_proto::cosmos::base::v1beta1::Coin {
        denom: c.denom,
        amount: c.amount.to_string(),
    };
    let osmo_d = osmosis_std::cosmos_sdk_proto::cosmos::base::v1beta1::Coin {
        denom: d.denom,
        amount: d.amount.to_string(),
    };
    // add msg_join_pool
    let msg_join_pool: CosmosMsg = MsgJoinPool {
        sender: config.admin.clone(),
        pool_id: vault_config.pool_id,
        share_out_amount: "1000".to_string(),
        token_in_maxs: vec![osmo_c],
    }
    .into();

    // burn stablecoins
    let burn_stables: CosmosMsg = MsgBurn {
        sender: config.admin,
        amount: Some(osmo_d),
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "liquidate")
        .add_message(msg_join_pool)
        .add_message(burn_stables))
}

pub fn try_deposit_collateral(
    deps: DepsMut<OsmosisQuery>,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let resp: primitives::nft::msg::OwnerOfResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: state.manager,
            msg: to_binary(&primitives::nft::msg::QueryMsg::OwnerOf {
                token_id: state.vault_id.to_string(),
                include_expired: Some(true),
            })?,
        }))?;

    if info.sender != resp.owner {
        return Err(ContractError::Unauthorized {});
    }
    let deposit = info.funds[0].clone();
    if state.collateral != deposit.denom {
        return Err(ContractError::NotRegisteredCollateral {
            registered: state.collateral,
            input: info.funds[0].clone().denom,
        });
    }

    Ok(Response::new()
        .add_attribute("method", "deposit_collateral")
        .add_attribute("denom", deposit.denom)
        .add_attribute("amount", deposit.amount.to_string()))
}

pub fn try_withdraw_collateral(
    deps: DepsMut<OsmosisQuery>,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let resp: primitives::nft::msg::OwnerOfResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: state.manager.clone(),
            msg: to_binary(&primitives::nft::msg::QueryMsg::OwnerOf {
                token_id: state.vault_id.to_string(),
                include_expired: Some(true),
            })?,
        }))?;

    if info.sender != resp.owner {
        return Err(ContractError::Unauthorized {});
    }

    let c = deps
        .querier
        .query_balance(&env.contract.address, state.collateral)?;
    let d = deps
        .querier
        .query_balance(&env.contract.address, state.debt.clone())?;

    let vault_config: VaultConfigResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: state.manager,
            msg: to_binary(&primitives::vault_manager::msg::QueryMsg::GetVaultConfig {
                clt: c.denom.clone(),
            })?,
        }))?;
    let spot_price = OsmosisQuery::spot_price(vault_config.pool_id, &c.denom, "g-usdc");
    let query = QueryRequest::from(spot_price);
    let c_price = deps.querier.query(&query)?;
    let spot_priced = OsmosisQuery::spot_price(vault_config.pool_id, &d.denom, "g-usdc");
    let query = QueryRequest::from(spot_priced);
    let d_price = deps.querier.query(&query)?;

    if !_is_valid_cdp(
        c_price,
        d_price,
        c.amount,
        state.borrow - amount,
        vault_config.c_decimal,
        vault_config.mcr,
    ) {
        return Err(ContractError::InvalidCDP {
            input: _cr(
                c_price,
                d_price,
                c.amount,
                state.borrow - amount,
                vault_config.c_decimal,
                vault_config.mcr,
            ),
            mcr: vault_config.mcr,
        });
    }
    Ok(Response::new()
        .add_attribute("method", "withdraw_collateral")
        .add_messages(vec![CosmosMsg::Bank(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![Coin {
                denom: state.debt,
                amount: amount,
            }],
        })]))
}

pub fn try_pay_debt(
    deps: DepsMut<OsmosisQuery>,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let resp: primitives::nft::msg::OwnerOfResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: state.manager.clone(),
            msg: to_binary(&primitives::nft::msg::QueryMsg::OwnerOf {
                token_id: state.vault_id.to_string(),
                include_expired: Some(true),
            })?,
        }))?;

    if info.sender != resp.owner {
        return Err(ContractError::Unauthorized {});
    }

    // check stablecoin input
    let deposit = info.funds[0].clone();
    if state.debt != deposit.denom {
        return Err(ContractError::NotRegisteredCollateral {
            registered: state.debt,
            input: info.funds[0].clone().denom,
        });
    }

    // get vault config and config
    let config: ConfigResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: state.manager.clone(),
        msg: to_binary(&primitives::vault_manager::msg::QueryMsg::GetConfig {})?,
    }))?;

    let vault_config: VaultConfigResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: state.manager.clone(),
            msg: to_binary(&primitives::vault_manager::msg::QueryMsg::GetVaultConfig {
                clt: state.collateral,
            })?,
        }))?;

    // process fees in sfr
    let spot_priced = OsmosisQuery::spot_price(vault_config.pool_id, &deposit.denom, "g-usdc");
    let query = QueryRequest::from(spot_priced);
    let d_price: Uint128 = deps.querier.query(&query)?;
    let d_value = d_price * deposit.amount;
    // (duration in months with 6 precision) * (sfr * assetValue/100(with 5decimals))
    let duration = ((env.block.time.seconds() - state.last_updated) * u64::pow(10, 6)) / 259200;
    let duration_v = (Uint128::from(duration) * d_value) / Uint128::from(u64::pow(10, 6));
    let fee = duration_v * Uint128::from(vault_config.sfr) / Uint128::from(10000000u64);
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.borrow = amount;
        state.last_updated = env.block.time.seconds();
        Ok(state)
    })?;

    let send_fee: CosmosMsg = CosmosMsg::Bank(BankMsg::Send {
        to_address: config.admin.to_string(),
        amount: vec![Coin {
            denom: deposit.denom.clone(),
            amount: fee,
        }],
    });

    let deduct_fee = deposit.amount - fee;

    let osmo_d = osmosis_std::cosmos_sdk_proto::cosmos::base::v1beta1::Coin {
        denom: deposit.denom.clone(),
        amount: deduct_fee.to_string(),
    };

    // burn stablecoins
    let burn_stables: CosmosMsg = MsgBurn {
        sender: config.admin,
        amount: Some(osmo_d),
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "pay_debt")
        .add_message(send_fee)
        .add_message(burn_stables))
}

pub fn try_borrow_more(
    deps: DepsMut<OsmosisQuery>,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let resp: primitives::nft::msg::OwnerOfResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: state.manager.clone(),
            msg: to_binary(&primitives::nft::msg::QueryMsg::OwnerOf {
                token_id: state.vault_id.to_string(),
                include_expired: Some(true),
            })?,
        }))?;

    if info.sender != resp.owner {
        return Err(ContractError::Unauthorized {});
    }
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.borrow = amount;
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("method", "borrow_more"))
}

//pub mod query;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetState {} => to_binary(&query_state(deps)?),
        QueryMsg::GetBalances {} => to_binary(&query_vault_balances(deps, env)?),
    }
}

fn query_state(deps: Deps) -> StdResult<StateResponse> {
    let state = STATE.load(deps.storage)?;

    let resp = StateResponse {
        vault_id: state.vault_id,
        manager: state.manager,
        collateral: state.collateral,
        debt: state.debt,
        v1: state.v1,
        borrow: state.borrow,
        last_updated: state.last_updated,
        sfr: state.ex_sfr,
    };
    Ok(resp)
}

fn query_vault_balances(deps: Deps, env: Env) -> StdResult<VaultBalanceResponse> {
    let state = STATE.load(deps.storage)?;

    let c = deps
        .querier
        .query_balance(&env.contract.address, state.collateral)?;
    let d = deps
        .querier
        .query_balance(&env.contract.address, state.debt.clone())?;

    let resp = VaultBalanceResponse { c, d };
    Ok(resp)
}
