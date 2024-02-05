#!/bin/sh

CODE=$1
POLICYHOLDER=$2
INSUREDPARTY=$3
DENOM=$4
BASERATE=$5
RATEPERMILEAGE=$6


INIT='{'\
'"policy_holder": "'"$POLICYHOLDER"'"'\
,'"insured_party": "'"$INSUREDPARTY"'"'\
,'"denom": "'"$DENOM"'"'\
,'"base_rate": '"$BASERATE"''\
,'"rate_per_mileage": '"$RATEPERMILEAGE"''\
'}';

$CHAIN tx wasm instantiate $CODE "$INIT" \
    --label "SOARCHAIN INSURANCE INIT" \
    --no-admin \
    --from runner \
    --node $NODE \
    --chain-id $CHAINID \
    --gas-prices 0.1$DENOM \
    --gas auto \
    --gas-adjustment 1.3 \
    -b block \
    -y

