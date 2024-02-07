#!/bin/sh

CONTRACT_ADDRESS=$1

CONFIG_QUERY='{"motus_by_address": {"address":"soar1ghfnkjlc5gxpldat7hm50tgggwc6l5h7ydwy2a"}}'

$CHAIN query wasm contract-state smart $CONTRACT_ADDRESS "$CONFIG_QUERY" --node $NODE --chain-id $CHAINID 
