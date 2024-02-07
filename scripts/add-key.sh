#!/bin/bash

ADDRESS="soar18a8u6s7zftzc4w8x7n6hdjfnlxlp7mr02c60tk"
ALLIANZ_ACCOUNT="allianz"
MNEMONIC="symbol ticket rocket math fresh wash law win thank scout husband guard aunt road essence magnet artwork check immense talent way lecture august frozen"

$CHAIN keys add $ALLIANZ_ACCOUNT --recover <<< $MNEMONIC

sleep 5

$CHAIN tx bank send $($CHAIN keys show -a challenger) $($CHAIN keys show -a $ALLIANZ_ACCOUNT) 1000000000$DENOM \
    --chain-id $CHAINID \
    --node $NODE \
    --gas-prices 0.1$DENOM \
    --gas auto \
    --gas-adjustment 1.3 \
    -b block \
    -y

sleep 5

$CHAIN q bank balances $($CHAIN keys show -a $ALLIANZ_ACCOUNT)


sleep 5

BOB_ACCOUNT="bob"
MNEMONIC="upset monster witness fiction word web bulb quarter vessel grab connect shop filter slam powder timber discover concert onion together road tissue icon mimic"

$CHAIN keys add $BOB_ACCOUNT --recover <<< $MNEMONIC

$CHAIN q bank balances $($CHAIN keys show -a $BOB_ACCOUNT)