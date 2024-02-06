#!/bin/sh

sudo docker stop soarchain-insurance-smart-contracts-validator-1
sudo docker rm soarchain-insurance-smart-contracts-validator-1

echo '@@@ soarchain-insurance-smart-contracts-validator-1 killed and removed'