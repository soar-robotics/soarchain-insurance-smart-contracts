#!/bin/sh

CODE=$1
ALLIANZ=$2
BOB=$3
BASERATE=$4
RATEPERMILEAGE=$5

INIT='{'\
'"policy_holder": "'"$(soarchaind keys show -a $ALLIANZ)"'"'\
,'"insured_party": "'"$(soarchaind keys show -a $BOB)"'"'\
,'"denom": "'"$DENOM"'"'\
,'"base_rate": '"$BASERATE"''\
,'"rate_per_mileage": '"$RATEPERMILEAGE"''\
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

