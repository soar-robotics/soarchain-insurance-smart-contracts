#!/bin/sh

sudo docker stop insurance-contract-validator-1
sudo docker rm insurance-contract-validator-1

echo '@@@ insurance-contract-validator-1 killed and removed'