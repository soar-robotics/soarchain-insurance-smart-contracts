#!/bin/sh

CODE=$1
ALLIANZ=$2

INIT='{'\
'"denom": "'"$DENOM"'"'\
'}';

$CHAIN tx wasm instantiate $CODE "$INIT" \
    --label "SOARCHAIN INSURANCE INIT" \
    --no-admin \
    --from $ALLIANZ \
    --node $NODE \
    --chain-id $CHAINID \
    --gas-prices 0.1$DENOM \
    --gas auto \
    --gas-adjustment 1.3 \
    -b block \
    -y

