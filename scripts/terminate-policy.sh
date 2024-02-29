#!/bin/sh

echo "-----------------------------------"
echo "We are going to terminate a policy:"
echo "-----------------------------------"

echo "Enter contract address:"
read contract

echo "Enter insured party account name:"
read party

echo "Enter dpr id:"
read dpr


ADDRESS=$(soarchaind keys show -a  $party)

TERMINATE_DATA='{"terminate":{"insured_party": "'"$ADDRESS"'", "dpr":"'"$dpr"'"}}'

$CHAIN tx wasm execute $contract "$TERMINATE_DATA" \
    --gas-prices 0.025$DENOM \
    --from $party \
    --node $NODE \
    --chain-id $CHAINID \
    --gas auto \
    --gas-adjustment 1.5 \
    -b block \
    -y