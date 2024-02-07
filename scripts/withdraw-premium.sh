#!/bin/sh

ID=$1
BOB=$2
CONTRACT=$3

WITHDRAW_DATA='{"withdraw":{"id": "'"$ID"'", "insured_party": "'"$($CHAIN keys show -a $BOB)"'"}}'


$CHAIN tx bank send $($CHAIN keys show -a $Bob) $CONTRACT 1000000$DENOM \
    --chain-id $CHAINID \
    --node $NODE \
    --gas-prices 0.1$DENOM \
    --gas auto \
    --gas-adjustment 1.3 \
    -b block \
    -y

sleep 5

$CHAIN q bank balances $CONTRACT

sleep 5

$CHAIN tx wasm execute $CONTRACT "$WITHDRAW_DATA" \
    --gas-prices 0.025$DENOM \
    --from $BOB \
    --node $NODE \
    --chain-id $CHAINID \
    --gas auto \
    --gas-adjustment 1.5 \
    -b block \
    -y