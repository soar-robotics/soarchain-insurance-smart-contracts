#!/bin/sh

CODE=$1
POLICY_HOLDER=$2
INSURED_PARTY=$3
DENOM=$4
BASERATE=$5
RATEPERMILEAGE=$6

echo $(soarchaind keys show -a runner)

INIT='{'\
'"policy_holder": "'"$(soarchaind keys show -a runner)"'"'\
,'"insured_party": "'"$(soarchaind keys show -a client)"'"'\
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

