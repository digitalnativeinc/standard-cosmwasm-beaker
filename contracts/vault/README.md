# CosmWasm Starter Pack for Osmosis

This is a template to build smart contracts in Rust to run inside a
[Cosmos SDK](https://github.com/cosmos/cosmos-sdk) module on Osmosis.
To understand the framework better, please read the overview in the
[cosmwasm repo](https://github.com/CosmWasm/cosmwasm/blob/master/README.md),
and dig into the [cosmwasm docs](https://www.cosmwasm.com).
This assumes you understand the theory and just want to get coding.

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
