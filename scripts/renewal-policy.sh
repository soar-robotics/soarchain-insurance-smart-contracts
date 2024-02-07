#!/bin/sh

ID=$1
BOB=$2
CONTRACT=$3

RENEWAL_DATA='{"renewal":{"id": "'"$ID"'", "premium":'"3000"', "duration":'"3"', "coverage":"'"60%"'", "insured_party": "'"$($CHAIN keys show -a $BOB)"'"}}'

$CHAIN tx wasm execute $CONTRACT "$RENEWAL_DATA" \
    --gas-prices 0.025$DENOM \
    --from $BOB \
    --node $NODE \
    --chain-id $CHAINID \
    --gas auto \
    --gas-adjustment 1.5 \
    -b block \
    -y