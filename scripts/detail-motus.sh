#!/bin/sh

echo "Enter contract address:"
read address

CONFIG_QUERY='{"motus_by_address": {"address":"soar1ghfnkjlc5gxpldat7hm50tgggwc6l5h7ydwy2a"}}'

$CHAIN query wasm contract-state smart $address "$CONFIG_QUERY" --node $NODE --chain-id $CHAINID 
