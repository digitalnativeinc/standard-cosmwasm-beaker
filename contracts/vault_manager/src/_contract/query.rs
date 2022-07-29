use crate::{
    msg::{ConfigResponse, VaultConfigResponse},
    state::{CONFIG, VAULTCONFIG},
};

use super::*;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetVaultConfig { clt } => to_binary(&query_vault_config(deps, clt)?),
        QueryMsg::GetConfig {} => to_binary(&query_config(deps)?),
        // QueryMsg::GetAssetPrice {} => to_binary(&query_asset_price(deps)?)
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
