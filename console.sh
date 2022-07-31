# test1 address: osmo1cyyzpxplxdzkeea7kwsydadg87357qnahakaks

# send fund to tokenfactory contract 

# issue stablecoin state
tf = contract.tokenfactory.signer(account.test1)

# create denom
await tf.createDenom({'subdenom': "usafu"}, "auto", "", [{'amount': '10000000', 'denom': 'uosmo'}])

# await tf.mintTokens({'amount': '1', 'denom': 'factory/{tokenfactory contract address}/usafu', 'mintToAddress': 'osmo1cyyzpxplxdzkeea7kwsydadg87357qnahakaks'})
await tf.mintTokens({'amount': '1', 'denom': 'factory/osmo1f8ehel3gphc3vaq028at9nckpgpa3d0frgaet6av2wfm7u6t4znq55ylp5/usafu', 'mintToAddress': 'osmo1cyyzpxplxdzkeea7kwsydadg87357qnahakaks'})

# change contract to call mint and burn
await tf.changeAdmin({'denom': 'factory/osmo1qg5ega6dykkxc307y25pecuufrjkxkaggkkxh7nad0vhyhtuhw3s0p34vn/safuu', 'newAdminAddress': 'osmo1cyyzpxplxdzkeea7kwsydadg87357qnahakaks'})

# your stablecoin address is bascially tokenfactory contract address


# set Vault manager config
vm = contract.vault_manager.signer(account.test1)

# await vm.initialize({'admin': 'osmo1cyyzpxplxdzkeea7kwsydadg87357qnahakaks', 'factory': '{Tokenfactory contract address}', 'stablecoin': '{stablecoin full denom}', 'v1': '{V1 contract address}', 'vaultCodeId' : 22})
await vm.initialize({'admin': 'osmo1cyyzpxplxdzkeea7kwsydadg87357qnahakaks', 'factory': 'osmo1f8ehel3gphc3vaq028at9nckpgpa3d0frgaet6av2wfm7u6t4znq55ylp5', 'stablecoin': 'factory/osmo1f8ehel3gphc3vaq028at9nckpgpa3d0frgaet6av2wfm7u6t4znq55ylp5/usafu', 'v1': 'osmo1w9j6668jjyc0p3gjlw8r5ea0j5q9ykpr2z5xgh6hh32l2jzjqxxs8xalp3', 'vaultCodeId' : 39})

await vm.setVaultConfig({'cDecimal': 6, 'clt': 'uosmo', 'lfr': 2000000, 'mcr': 20000000, 'poolId': 1, 'sfr': 100000000})


# Authorize token mint in admin
await tf.changeAdmin({'denom': 'factory/osmo1wqchrjh07e3kxaee59yrpzckwr94j03zchmdslypvkv6ps0684msrzv484/usafu2', 'newAdminAddress': 'osmo1t4a34yj7r7h7fffuls3gvj6ept6vqyuujm4v2a4lnrnm8yckwmuqnkd0a9'})


let result = await vm.createVault({'dAmount': '100000'}, "auto", "", [{'amount': '10000000', 'denom': 'uosmo'}])

# get vault contract address

console.dir(result, {depth: null})


# Interact with vault