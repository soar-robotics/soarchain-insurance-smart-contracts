#!/bin/sh

CONTRACT_ADDRESS=$1
ID=$2

$CHAIN query wasm contract-state smart $CONTRACT_ADDRESS '{"details":{"id":"'"$ID"'"}}' \
    --node $NODE \
    --chain-id $CHAINID 
