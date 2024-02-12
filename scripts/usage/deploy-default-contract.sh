#!/bin/sh

ALLIANZ=$1

# Store the smart contract
$CHAIN tx wasm store "./artifacts/traditional.wasm" \
    --from $ALLIANZ \
    --node $NODE \
    --chain-id $CHAINID \
    --gas-prices 0.1$DENOM \
    --gas auto \
    --gas-adjustment 1.3 \
    -b block \
    -y