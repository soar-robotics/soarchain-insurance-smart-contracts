#!/bin/sh

echo "We are going to renew a policy:"
echo "Enter contract address:"
read contract
echo "Enter insured party account name:"
read party
echo "Enter premium amount:"
read premium
echo "Enter new duration of policy:"
read duration


RENEWAL_DATA='{"renewal":{"premium":'"$premium"', "duration":'"$duration"', "insured_party": "'"$($CHAIN keys show -a $party)"'"}}'

$CHAIN tx wasm execute $contract "$RENEWAL_DATA" \
    --gas-prices 0.025$DENOM \
    --from $party \
    --node $NODE \
    --chain-id $CHAINID \
    --gas auto \
    --gas-adjustment 1.5 \
    -b block \
    -y