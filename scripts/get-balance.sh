#!/bin/sh

echo "We are going to fetch balance of an address:"
echo "Enter the address (account/contract):"
read account

$CHAIN q bank balances $account