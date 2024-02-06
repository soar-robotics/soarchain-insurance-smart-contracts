#!/bin/sh

ID=$1
INSURED_PARTY=$2
CONTRACT=$3

RENEWAL_DATA='{"renewal":{"id": "'"$ID"'", "premium":'"3000"', "duration":'"3"', "coverage":"'"60%"'", "insured_party": "'"$($CHAIN keys show -a $INSURED_PARTY)"'"}}'

$CHAIN tx wasm execute $CONTRACT "$RENEWAL_DATA" \
    --gas-prices 0.025$DENOM \
    --from $INSURED_PARTY \
    --node $NODE \
    --chain-id $CHAINID \
    --gas auto \
    --gas-adjustment 1.5 \
    -b block \
    -y