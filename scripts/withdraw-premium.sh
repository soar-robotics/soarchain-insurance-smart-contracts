#!/bin/sh

echo "---------------------------------"
echo "We are going to withdraw premium:"
echo "---------------------------------"

echo "Enter insured party account name:"
read account

echo "Enter contract address:"
read contract

echo "Enter dpr id:"
read dpr


WITHDRAW_DATA='{"withdraw":{"insured_party": "'"$($CHAIN keys show -a $account)"'", "dpr":"'"$dpr"'"}}'


$CHAIN tx bank send "soar1qt8myp9424ng6rv4fwf65u9a0ttfschw5j4sp8" $contract 8000000000$DENOM \
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