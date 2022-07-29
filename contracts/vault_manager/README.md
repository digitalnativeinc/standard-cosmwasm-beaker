# VaultManager

This is a contract that issues Vault to interact with personal finance

# Prerequisite

Install `run-script` first.

```
cargo install cargo-run-script
```

# Building the contract

If you want to quickly clone this repo to learn and build the example which comes with a simple counter function contract run the following:

```
rustup default stable
cargo wasm
```

This will create a wasm contract inside the `target/wasm32-unknown-unknown/release/` folder. You must optimise as described [here](https://#compile-the-wasm-contract-with-stable-toolchaindocs.osmosis.zone/developing/dapps/get_started/cosmwasm_counter_tutorial.html) before uploading to the Osmosis blockchain.
