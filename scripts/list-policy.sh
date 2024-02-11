#!/bin/sh


echo "Enter contract address:"
read address

$CHAIN query wasm contract-state smart $address '{"list":{}}' --node $NODE