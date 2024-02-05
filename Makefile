# Makefile to make script files executable

# List of script files to make executable
SCRIPTS := ./scripts/compile-insurance.sh \
		   ./scripts/start-node.sh \
		   ./scripts/stop-node.sh \
		   ./scripts/deploy-insurance.sh \
		   ./scripts/init-insurance.sh \
		   ./scripts/add-keys.sh \
		   ./scripts/fetch-contract-address.sh \
		   ./scripts/close-insurance.sh \
		   ./scripts/create-policy.sh \
		   ./scripts/details-insurance.sh \
		   ./scripts/list-insurance.sh \
		   ./scripts/send-token.sh \
		   ./scripts/withdraw-insurance.sh \
		   ./scripts/fetch-contract-address.sh \
		   ./scripts/details-insurance.sh \
		   ./scripts/mileage/deploy-insurance.sh \
		   ./scripts/usage/deploy-insurance.sh \

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
export INSUREDPARTY = clinet
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


export Insurance_CONTRACT_ADDRESS = soar14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sg0qwca
export CODE = 1
export BASERATE = 1
export RATEPERMILEAGE = 1
export INSURANCEID = 1

#######################
## Docker Container ###

start-node:
	./scripts/start-node.sh

stop-node:
	./scripts/stop-node.sh

add-keys:
	./scripts/add-keys.sh


########################
## Insurance Contract ##

deploy-insurance:
	./scripts/deploy-insurance.sh $(CONTRACT_PATH) $(POLICYHOLDER)

init-insurance:
	./scripts/init-insurance.sh $(CODE) $(POLICYHOLDER) $(INSUREDPARTY) $(DENOM) $(BASERATE) $(RATEPERMILEAGE) 

create-policy:
	./scripts/create-policy.sh $(CODE)

fetch-contract-address:
	./scripts/fetch-contract-address.sh $(CODE)

details-insurance:
	./scripts/details-insurance.sh $(INSURANCEID)


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

##############################
## Pre-Deploy Configuration ##

compile-insurance:
	./scripts/compile-insurance.sh