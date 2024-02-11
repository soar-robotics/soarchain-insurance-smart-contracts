
#!/bin/sh

CONTRACT=$1
AMOUNT=$2

echo "We are going to send some token to contract address:"
echo "Enter insured party account name:"
read account
echo "Enter your contract address:"
read contract
echo "Enter the amount:"
read amount

$CHAIN tx bank send $($CHAIN keys show -a $account) $contract $amount$DENOM \
    --chain-id $CHAINID \
    --node $NODE \
    --gas-prices 0.1$DENOM \
    --gas auto \
    --gas-adjustment 1.3 \
    -b block \
    -y

sleep 5

$CHAIN q bank balances $contract