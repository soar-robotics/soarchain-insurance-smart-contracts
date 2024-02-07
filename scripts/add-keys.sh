#!/bin/sh


# Assign variables
MASTER_ACCOUNT="soarMasterAccount"
POLICY_HOLDER="holder"
INSURED_PARTY="insured"  


# Create and fund soarMasterAccount
$CHAIN keys add $MASTER_ACCOUNT --recover
sleep 5
$CHAIN tx bank send $($CHAIN keys show -a apollo) $($CHAIN keys show -a $MASTER_ACCOUNT) 100000000$DENOM \
    --chain-id $CHAINID \
    --node $NODE \
    --gas-prices 0.1$DENOM \
    --gas auto \
    --gas-adjustment 1.3 \
    -b block \
    -y

sleep 5
$CHAIN tx bank send $($CHAIN keys show -a $MASTER_ACCOUNT) $Insurance_CONTRACT_ADDRESS 100000$DENOM \
    --chain-id $CHAINID \
    --node $NODE \
    --gas-prices 0.1$DENOM \
    --gas auto \
    --gas-adjustment 1.3 \
    -b block \
    -y

# Create and fund holder account
$CHAIN keys add $POLICY_HOLDER --recover
sleep 5
$CHAIN tx bank send $($CHAIN keys show -a $MASTER_ACCOUNT) $($CHAIN keys show -a $POLICY_HOLDER) 100000$DENOM \
    --chain-id $CHAINID \
    --node $NODE \
    --gas-prices 0.1$DENOM \
    --gas auto \
    --gas-adjustment 1.3 \
    -b block \
    -y

sleep 5
# Create and fund insured account
$CHAIN keys add $INSURED_PARTY --recover
sleep 5
$CHAIN tx bank send $($CHAIN keys show -a $MASTER_ACCOUNT) $($CHAIN keys show -a $INSURED_PARTY) 100000$DENOM \
    --chain-id $CHAINID \
    --node $NODE \
    --gas-prices 0.1$DENOM \
    --gas auto \
    --gas-adjustment 1.3 \
    -b block \
    -y


