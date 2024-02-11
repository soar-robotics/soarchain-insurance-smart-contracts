#!/bin/sh

echo "We are going to create an account for you:"
echo "Enter insurer account name:"
read account
echo "Enter contract code:"
read code

echo $account

INIT='{'\
'"denom": "'"$DENOM"'"'\
,'"insurer": "'"$account"'"'\
'}';

$CHAIN tx wasm instantiate $code "$INIT" \
    --label "SOARCHAIN INSURANCE INIT" \
    --no-admin \
    --from $account \
    --node $NODE \
    --chain-id $CHAINID \
    --gas-prices 0.1$DENOM \
    --gas auto \
    --gas-adjustment 1.3 \
    -b block \
    -y

