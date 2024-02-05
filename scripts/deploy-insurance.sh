#!/bin/sh

CONTRACT_PATH=$1
POLICYHOLDER=$2

# Store the smart contract
$CHAIN tx wasm store $CONTRACT_PATH \
    --from $POLICYHOLDER \
    --node $NODE \
    --chain-id $CHAINID \
    --gas-prices 0.1$DENOM \
    --gas auto \
    --gas-adjustment 1.3 \
    -b block \
    -y