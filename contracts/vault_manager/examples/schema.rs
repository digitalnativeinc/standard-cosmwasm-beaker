use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use vault_manager::{msg::{ConfigResponse, VaultConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg}, state::{Config, VaultConfig, Reserve}};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema/vault_manager");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(Config), &out_dir);
    export_schema(&schema_for!(VaultConfig), &out_dir);
    export_schema(&schema_for!(Reserve), &out_dir);
    export_schema(&schema_for!(VaultConfigResponse), &out_dir);
    export_schema(&schema_for!(ConfigResponse), &out_dir);
}
