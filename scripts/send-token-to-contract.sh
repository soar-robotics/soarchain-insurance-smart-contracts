
#!/bin/sh

CONTRACT=$1
AMOUNT=$2

$CHAIN tx bank send $($CHAIN keys show -a $Allianz) $CONTRACT $AMOUNT$DENOM \
    --chain-id $CHAINID \
    --node $NODE \
    --gas-prices 0.1$DENOM \
    --gas auto \
    --gas-adjustment 1.3 \
    -b block \
    -y

sleep 5

$CHAIN q bank balances $CONTRACT