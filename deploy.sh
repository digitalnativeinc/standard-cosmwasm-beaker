beaker wasm deploy vault_manager --signer-account test1 --no-wasm-opt 
beaker wasm deploy vault --signer-account test1 --no-wasm-opt 
beaker wasm deploy tokenfactory --signer-account test1 --no-wasm-opt 
# beaker wasm deploy nft --signer-account test1 --no-wasm-opt --raw '{"name": "VaultOne", "symbol": "V1", "minter": "{vault_manager contract address}"}'
beaker wasm deploy nft --signer-account test1 --no-wasm-opt --raw '{"name": "VaultOne", "symbol": "V1", "minter": "osmo1t4a34yj7r7h7fffuls3gvj6ept6vqyuujm4v2a4lnrnm8yckwmuqnkd0a9"}'

