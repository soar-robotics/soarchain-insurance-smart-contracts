#!/bin/bash

echo "We are going to create an account for you:"
echo "Enter your account name:"
read account
echo "Enter your mnemonic:"
read mnemonic


$CHAIN keys add $account --recover <<< $mnemonic

sleep 5

echo "We are going to send some token to your account for doing tests:"

$CHAIN tx bank send "soar1qt8myp9424ng6rv4fwf65u9a0ttfschw5j4sp8" $($CHAIN keys show -a $account) 1000000000$DENOM \
    --chain-id $CHAINID \
    --node $NODE \
    --gas-prices 0.1$DENOM \
    --gas auto \
    --gas-adjustment 1.3 \
    -b block \
    -y

sleep 5

$CHAIN q bank balances $($CHAIN keys show -a $account)

