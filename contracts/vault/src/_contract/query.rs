use crate::msg::{StateResponse, VaultBalanceResponse};

use super::*;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
    QueryMsg::GetState {  } => to_binary(&query_state(deps)?),
    QueryMsg::GetBalances { } => to_binary(&query_vault_balances(deps, env)?)
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

    let resp = VaultBalanceResponse{
        c,
        d
    };
    Ok(resp)
}
