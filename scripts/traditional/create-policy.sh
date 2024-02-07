#!/bin/sh

CONTRACT_ADDRESS=$1
POLICY_ID=$2

EXECUTE_CREATE_POLICY='{"create_policy":{"id": "'"$POLICY_ID"'", "policy_holder":"'"$($CHAIN keys show -a $Allianz)"'", "insured_party":"'"$($CHAIN keys show -a $Bob)"'", "start_date":'"2200"', "beneficiary":"'"beneficiary"'", "coverage":"'"50"'", "plan":"'"plan"'", "premium":'"2000"', "duration":'"3"', "termination_date":'"0"',"is_active":'"false"',"closed":'"false"'}}'

$CHAIN tx wasm execute $CONTRACT_ADDRESS "$EXECUTE_CREATE_POLICY" \
    --node $NODE \
    --chain-id $CHAINID \
    --gas-prices 0.025$DENOM \
    --gas auto \
    --gas-adjustment 1.5 \
    --from $Allianz \
    -b block \
    -y

