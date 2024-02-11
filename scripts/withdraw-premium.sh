#!/bin/sh

echo "We are going to withdraw premium:"
echo "Enter insured party account name:"
read account
echo "Enter contract address:"
read contract

WITHDRAW_DATA='{"withdraw":{"insured_party": "'"$($CHAIN keys show -a $account)"'"}}'


$CHAIN tx bank send $($CHAIN keys show -a $account) $contract 1000000$DENOM \
    --chain-id $CHAINID \
    --node $NODE \
    --gas-prices 0.1$DENOM \
    --gas auto \
    --gas-adjustment 1.3 \
    -b block \
    -y

sleep 5

$CHAIN q bank balances $contract

sleep 5

$CHAIN tx wasm execute $contract "$WITHDRAW_DATA" \
    --gas-prices 0.025$DENOM \
    --from $account \
    --node $NODE \
    --chain-id $CHAINID \
    --gas auto \
    --gas-adjustment 1.5 \
    -b block \
    -y