# Makefile to make script files executable

# List of script files to make executable
SCRIPTS := ./scripts/compile-insurance.sh \
		   ./scripts/start-node.sh \
		   ./scripts/stop-node.sh \
		   ./scripts/init-insurance.sh \
		   ./scripts/add-keys.sh \
		   ./scripts/fetch-contract-address.sh \
		   ./scripts/create-policy.sh \
		   ./scripts/details-insurance.sh \
		   ./scripts/fetch-contract-address.sh \
		   ./scripts/details-insurance.sh \
		   ./scripts/mileage/deploy-insurance.sh \
		   ./scripts/usage/deploy-insurance.sh \
		   ./scripts/traditional/deploy-insurance.sh \
		   ./scripts/get-balance.sh \
		   ./scripts/withdraw-premium.sh \
		   ./scripts/send-token-to-contract.sh \
		   ./scripts/renewal-policy.sh \
		   ./scripts/terminate-policy.sh \
		   ./scripts/add-key.sh \

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
export POLICYHOLDER = runner
export INSUREDPARTY = client
export CONTRACT_PATH="./artifacts/insurance.wasm"

############
## DEVNET ##
############
# export NODE = http://164.92.252.231:26657
# export CHAINID = soarchaintestnet
# export DENOM = utmotus
# export CHAIN = soarchaind
# export POLICYHOLDER = holder
# export InsuredParty = insured


export CODE = 1
export BASERATE = 1
export RATEPERMILEAGE = 1
export INSURANCEID = 1
export Insurance_CONTRACT_ADDRESS = soar14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sg0qwca
export POLICY_ID = 1

#######################
## Docker Container ###

start-node:
	./scripts/start-node.sh

stop-node:
	./scripts/stop-node.sh

add-keys:
	./scripts/add-keys.sh

get-balance:
	./scripts/get-balance.sh $(POLICYHOLDER)

add-key:
	./scripts/add-key.sh

AMOUNT=100000
make-payment:
	./scripts/send-token-to-contract.sh $(Insurance_CONTRACT_ADDRESS) $(AMOUNT)


####################################
## Traditional Insurance Contract ##

deploy-insurance:
	./scripts/traditional/deploy-insurance.sh $(POLICYHOLDER)

# This script initiates various types of contracts.
# Ensure to update the $(CODE) variable with the address of the latest deployed contract.
init-insurance:
	./scripts/init-insurance.sh $(CODE) $(POLICYHOLDER) $(INSUREDPARTY) $(DENOM) $(BASERATE) $(RATEPERMILEAGE) 


# This script initiates traditional types of contracts.
# Ensure to update the $(CONTRACT_ADDRESS) & $(POLICY_ID) variables with the id and the address of the latest deployed contract.
create-policy:
	./scripts/traditional/create-policy.sh $(Insurance_CONTRACT_ADDRESS) $(POLICY_ID)

fetch-contract-address:
	./scripts/fetch-contract-address.sh $(CODE)

# This script query various types of policies.
# Make sure to update the $(POLICY_ID) variable with the ID of the desired created policy.
details-insurance:
	./scripts/details-insurance.sh $(Insurance_CONTRACT_ADDRESS) $(POLICY_ID)


withdraw-premium:
	./scripts/withdraw-premium.sh  $(POLICY_ID) $(INSUREDPARTY) $(Insurance_CONTRACT_ADDRESS)

renewal-policy:
	./scripts/renewal-policy.sh  $(POLICY_ID) $(INSUREDPARTY) $(Insurance_CONTRACT_ADDRESS)


################################
## Usage Based Smart Contract ##

deploy_usage_based_insurance:
	./scripts/mileage/deploy-insurance.sh $(POLICYHOLDER)

create-usage-based-policy:
	./scripts/usage/create-policy.sh $(CODE)


##################################
## Mileage Based Smart Contract ##


deploy_mileage_based_insurance:
	./scripts/mileage/deploy-insurance.sh $(POLICYHOLDER)

create-mileage-based-policy:
	./scripts/mileage/create-policy.sh $(CODE)


# This script initiates usage_based & mileage_based types of contracts.
# Ensure to update the $(CONTRACT_ADDRESS) & $(POLICY_ID) variables with the id and the address of the latest deployed contract.
UBI_POLICY_ID = 2
UBI_CONTRACT_ADDRESS = soar18v47nqmhvejx3vc498pantg8vr435xa0rt6x0m6kzhp6yuqmcp8szu7mgm
create-UBI-policy:
	./scripts/create-policy.sh $(UBI_CONTRACT_ADDRESS) $(UBI_POLICY_ID)


terminate-policy:
	./scripts/terminate-policy.sh $(POLICY_ID) $(Insurance_CONTRACT_ADDRESS)


##############################
## Pre-Deploy Configuration ##

compile-insurance:
	./scripts/compile-insurance.sh