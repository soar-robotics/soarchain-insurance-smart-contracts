#!/bin/sh

echo "Enter contract address:"
read contract
echo "Enter insured party account name:"
read party


ADDRESS=$(soarchaind keys show -a  $party)

TERMINATE_DATA='{"terminate":{"insured_party": "'"$ADDRESS"'"}}'

$CHAIN tx wasm execute $contract "$TERMINATE_DATA" \
    --gas-prices 0.025$DENOM \
    --from $party \
    --node $NODE \
    --chain-id $CHAINID \
    --gas auto \
    --gas-adjustment 1.5 \
    -b block \
    -y