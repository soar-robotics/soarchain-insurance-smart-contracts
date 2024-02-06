#!/bin/sh

ID=$1
CONTRACT=$2

TERMINATE_DATA='{"terminate":{"id": "'"$ID"'"}}'

$CHAIN tx wasm execute $CONTRACT "$TERMINATE_DATA" \
    --gas-prices 0.025$DENOM \
    --from $POLICYHOLDER \
    --node $NODE \
    --chain-id $CHAINID \
    --gas auto \
    --gas-adjustment 1.5 \
    -b block \
    -y