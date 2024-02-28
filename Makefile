# List of script files to make executable
SCRIPTS := ./scripts/compile-contract.sh \
		   ./scripts/start-node.sh \
		   ./scripts/stop-node.sh \
		   ./scripts/init-contract.sh \
		   ./scripts/add-keys.sh \
		   ./scripts/fetch-default-contract-address.sh \
		   ./scripts/fetch-contract-address.sh \
		   ./scripts/create-policy.sh \
		   ./scripts/detail-policy.sh \
		   ./scripts/mileage/deploy-contract.sh \
		   ./scripts/mileage/deploy-default-contract.sh \
		   ./scripts/mileage/create-policy.sh \
		   ./scripts/usage/deploy-contract.sh \
		   ./scripts/usage/deploy-default-contract.sh \
		   ./scripts/usage/create-policy.sh \
		   ./scripts/traditional/create-policy.sh \
		   ./scripts/traditional/create-default-policy.sh \
		   ./scripts/traditional/deploy-contract.sh \
		   ./scripts/traditional/deploy-default-contract.sh  \
		   ./scripts/get-balance.sh \
		   ./scripts/withdraw-premium.sh \
		   ./scripts/send-token-to-contract.sh \
		   ./scripts/renew-policy.sh \
		   ./scripts/terminate-policy.sh \
		   ./scripts/add-key.sh \
		   ./scripts/detail-motus.sh \
		   ./scripts/list-policy.sh \
		   ./scripts/init-default-contract.sh


# Target to make all script files executable
make-scripts-executable:
	@chmod +x $(SCRIPTS)
	@echo "Script files are now executable."

# Default target
.PHONY: make-scripts-executable


###########
## LOCAL ##
###########
export CHAIN = soarchaind
export NODE = http://localhost:26657
export CHAINID = soarchaindevnet
export DENOM = udmotus
export Allianz = allianz # Insurance company as insurer
export Bob = bob # Motus client which is already registered as insured party
export CONTRACT_PATH="./artifacts/insurance.wasm"

############
## DEVNET ##
############
# export NODE = http://164.92.252.231:26657
# export CHAINID = soarchaintestnet
# export DENOM = utmotus
# export CHAIN = soarchaind
# export Allianz = allianz
# export Bob = bob


export CODE = 1
export BASERATE = 1
export RATEPERMILEAGE = 1
export INSURANCEID = 1
export Insurance_CONTRACT_ADDRESS = soar14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sg0qwca

#######################
## Docker Container ###

start-node:
	./scripts/start-node.sh

stop-node:
	./scripts/stop-node.sh

add-keys:
	./scripts/add-keys.sh

get-balance-default-account:
	./scripts/get-balance-default-account.sh $(Allianz)

get-balance:
	./scripts/get-balance.sh

add-key:
	./scripts/add-key.sh

make-payment:
	./scripts/send-token-to-contract.sh


####################################
## Deploy  Contracts ##

deploy-default-contract:
	./scripts/traditional/deploy-default-contract.sh $(Allianz)

deploy_default_usage_contract:
	./scripts/mileage/deploy-default-contract.sh $(Allianz)

deploy_default_mileage_contract:
	./scripts/mileage/deploy-default-contract.sh $(Allianz)

deploy-contract:
	./scripts/traditional/deploy-contract.sh

deploy-usage:
	./scripts/usage/deploy-contract.sh 

deploy-mileage:
	./scripts/mileage/deploy-contract.sh 


####################################
## Initiate  Contracts ##

# This script initiates various types of contracts.
# Ensure to update the $(CODE) variable with the address of the latest deployed contract.
init-default-contract:
	./scripts/init-default-contract.sh $(CODE) $(Allianz)

init:
	./scripts/init-contract.sh

################################
## Create Contract ##

# This script initiates traditional types of contracts.
# Ensure to update the $(CONTRACT_ADDRESS) & $(POLICY_ID) variables with the id and the address of the latest deployed contract.
create-default-policy:
	./scripts/traditional/create-default-policy.sh $(Insurance_CONTRACT_ADDRESS) $(Bob)

create-policy:
	./scripts/traditional/create-policy.sh

create-usage:
	./scripts/usage/create-policy.sh

create-mileage:
	./scripts/mileage/create-policy.sh


##################################
## Fetch Contract ##


fetch-default-contract-address:
	./scripts/fetch-default-contract-address.sh $(CODE)

fetch-contract-address:
	./scripts/fetch-contract-address.sh

# This script query various types of policies.
# Make sure to update the $(POLICY_ID) variable with the ID of the desired created policy.
detail-policy:
	./scripts/detail-policy.sh

detail-motus:
	./scripts/detail-motus.sh

list-Default-policy:
	./scripts/list-default-policy.sh $(Insurance_CONTRACT_ADDRESS)

list-policy:
	./scripts/list-policy.sh

withdraw-premium:
	./scripts/withdraw-premium.sh

renew-policy:
	./scripts/renew-policy.sh


# This script initiates usage_based & mileage_based types of contracts.
# Ensure to update the $(CONTRACT_ADDRESS) & $(POLICY_ID) variables with the id and the address of the latest deployed contract.
UBI_POLICY_ID = 2
UBI_CONTRACT_ADDRESS = soar18v47nqmhvejx3vc498pantg8vr435xa0rt6x0m6kzhp6yuqmcp8szu7mgm
create-UBI-policy:
	./scripts/create-policy.sh $(UBI_CONTRACT_ADDRESS) $(UBI_POLICY_ID)


terminate-policy:
	./scripts/terminate-policy.sh $(Bob) $(Insurance_CONTRACT_ADDRESS)


##############################
## Pre-Deploy Configuration ##

compile:
	./scripts/compile-contract.sh
