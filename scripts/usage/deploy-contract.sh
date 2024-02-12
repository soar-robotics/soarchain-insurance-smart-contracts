#!/bin/sh

echo "We are going to deploy a contract for you:"
echo "Enter insurer account name:"
read account

# Store the smart contract
$CHAIN tx wasm store "./artifacts/usage.wasm" \
    --from $account \
    --node $NODE \
    --chain-id $CHAINID \
    --gas-prices 0.1$DENOM \
    --gas auto \
    --gas-adjustment 1.3 \
    -b block \
    -y