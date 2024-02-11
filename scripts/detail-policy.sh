#!/bin/sh


echo "We are going to fetch a policy by insured party account:"
echo "Enter insured party account name:"
read account
echo "Enter contract address:"
read contract


$CHAIN query wasm contract-state smart $contract '{"details":{"address":"'"$($CHAIN keys show -a $account)"'"}}' \
    --node $NODE \
    --chain-id $CHAINID 
