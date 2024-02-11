#!/bin/sh


echo "We are going to fetch a contract by contract code:"
echo "Enter your contract code:"
read code

$CHAIN query wasm list-contract-by-code $code