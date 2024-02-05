#!/bin/sh

EXECUTE_CREATE_POLICY='{"create_policy":{"id": "'"1"'", "policy_holder":"'"$POLICYHOLDER"'", "insured_party":"'"$INSUREDPARTY"'", "creation_date":'"2400"', "beneficiary":"'"beneficiary"'", "coverage":"'"50"'", "plan":"'"plan"'", "premium":'"2400"', "period":'"12"', "closed":'"false"'}}'

$CHAIN tx wasm execute $Insurance_CONTRACT_ADDRESS "$EXECUTE_CREATE_POLICY" \
    --node $NODE \
    --chain-id $CHAINID \
    --gas-prices 0.025$DENOM \
    --gas auto \
    --gas-adjustment 1.5 \
    --from $POLICYHOLDER \
    -b block \
    -y
