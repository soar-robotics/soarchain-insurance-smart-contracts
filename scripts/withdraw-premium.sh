#!/bin/sh

ID=$1
INSURED_PARTY=$2
CONTRACT=$3

WITHDRAW_DATA='{"withdraw":{"id": "'"$ID"'", "insured_party": "'"$($CHAIN keys show -a $INSURED_PARTY)"'"}}'

$CHAIN tx wasm execute $CONTRACT "$WITHDRAW_DATA" \
    --gas-prices 0.025$DENOM \
    --from $INSURED_PARTY \
    --node $NODE \
    --chain-id $CHAINID \
    --gas auto \
    --gas-adjustment 1.5 \
    -b block \
    -y