#!/bin/sh

ID=$1

$CHAIN query wasm contract-state smart $Insurance_CONTRACT_ADDRESS '{"details":{"id":"'"$ID"'"}}' \
    --node $NODE \
    --chain-id $CHAINID 
