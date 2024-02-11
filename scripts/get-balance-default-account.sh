#!/bin/sh

RECEIPT=$1

$CHAIN q bank balances $($CHAIN keys show -a $RECEIPT)