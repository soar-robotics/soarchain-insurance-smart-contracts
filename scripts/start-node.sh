#!/bin/bash

sudo docker compose -f docker-compose.yml up

sleep 15

ADDRESS="soar1ghfnkjlc5gxpldat7hm50tgggwc6l5h7ydwy2a"
BOB_ACCOUNT="bob"
MNEMONIC="upset monster witness fiction word web bulb quarter vessel grab connect shop filter slam powder timber discover concert onion together road tissue icon mimic"

$CHAIN keys add $BOB_ACCOUNT --recover <<< $MNEMONIC

sleep 5

$CHAIN q bank balances $($CHAIN keys show -a $BOB_ACCOUNT)