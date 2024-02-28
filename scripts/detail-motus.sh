#!/bin/sh

echo "Enter contract address:"
read address

echo "Enter insured party address:"
read party

echo "Enter dpr id:"
read dpr

CONFIG_QUERY='{"motus_by_address": {"address":"'"$party"'", "dpr":"'"$dpr"'"}}'

$CHAIN query wasm contract-state smart $address "$CONFIG_QUERY" --node $NODE --chain-id $CHAINID 
