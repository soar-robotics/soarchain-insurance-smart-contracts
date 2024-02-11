#!/bin/sh

echo "We are going to create a liability policy for your contract:"
echo "Enter contract address:"
read contract
echo "Enter insurer account name:"
read account
echo "Enter insured party account name:"
read party
echo "Enter duration of a liability policy:"
read duration
echo "Enter laibility limits. Example "'"100/300/50"'". It includes 3 limits for the liability policy. 1- limits_bodily_injury_per_person, 2- limits_bodily_injury_per_accident 3- limits_property_damage:"
read limits
echo "Enter deductible amount:"
read deductible
echo "Enter insured party age:"
read age
echo "Enter claims free years. Max is 7"
read years

EXECUTE_CREATE_POLICY='{"create_liability_policy":{
     "insurer":"'"$($CHAIN keys show -a $account)"'",
     "insured_party":"'"$($CHAIN keys show -a $party)"'", 
     "duration":'"$duration"', 
     "document_hash":"e0d123e5f316bef78bfdf5a008837577", 
     "liability_limit":'"$limits"',
     "vehicle_type":"sedan",
     "deductible_amount": '"$deductible"',
     "risk_point":{"age": '"$age"', "location":"paris"},
     "driving_history":{"consecutive_claim_free_years": '"$years"'}
  }
}';
#2EXECUTE_CREATE_POLICY='{"create_liability_policy":{"id": "'"$POLICY_ID"'","vehicle":{"vin":"jgjhgj"}, "insurer":"'"$($CHAIN keys show -a $Allianz)"'", "insured_party":"'"$($CHAIN keys show -a $INSURED_PARTY)"'","document_hash":"hjgjhg", "start_time":'"2200"', "terms":{"coverage":{"liability":{"limits_bodily_injury_per_person":'"4555"',"limits_bodily_injury_per_accident":'"888"',"limits_property_damage":'"67567"'},"comprehensive":{"limit_stolen":'"89"',"limit_damaged":'"234"'},"collision":{"limit":'"456"'}},"frequency_of_premium":"daily","exclusions":["racing"],"usage_restrictions":"gfhfgh","claims_process":"fghfgh","deductibles":{"collision_coverage":'"78678"',"comprehensive_coverage":'"678678"'},"endorsements":{"sound_system":'"4354"',"change_of_vehicle":'"456456"',"increase_in_coverage_limits":'"56564"',"change_in_usage":'"4564"'},"policy_period":{"from":"gjghjg","to":"ghjghj"}},"risk_point":{"location":"paris", "driving_record":"80", "age":'"40"'}, "premium":'"2000"', "duration":'"3"', "termination_time":'"0"',"is_active":'"false"',"closed":'"false"'}}'

# EXECUTE_CREATE_POLICY='{"create_policy":{"id": "'"$POLICY_ID"'","vehicle":{"vin":"jgjhgj"}, "insurer":"'"$($CHAIN keys show -a $Allianz)"'", "insured_party":"'"$($CHAIN keys show -a $INSURED_PARTY)"'","document_hash":"hjgjhg", "start_time":'"2200"', "terms":{"coverage":{"liability":{"limits_bodily_injury_per_person":'"4555"',"limits_bodily_injury_per_accident":'"888"',"limits_property_damage":'"67567"'},"comprehensive":{"limit_stolen":'"89"',"limit_damaged":'"234"'},"collision":{"limit":'"456"'}},"frequency_of_premium":"daily","exclusions":["racing"],"usage_restrictions":"gfhfgh","claims_process":"fghfgh","deductibles":{"collision_coverage":'"78678"',"comprehensive_coverage":'"678678"'},"endorsements":{"sound_system":'"4354"',"change_of_vehicle":'"456456"',"increase_in_coverage_limits":'"56564"',"change_in_usage":'"4564"'},"policy_period":{"from":"gjghjg","to":"ghjghj"}},"risk_point":{"location":"paris", "driving_record":"80", "age":'"40"'}, "premium":'"2000"', "duration":'"3"', "termination_time":'"0"',"is_active":'"false"',"closed":'"false"'}}'
$CHAIN tx wasm execute $contract "$EXECUTE_CREATE_POLICY" \
    --node $NODE \
    --chain-id $CHAINID \
    --gas-prices 0.025$DENOM \
    --gas auto \
    --gas-adjustment 1.5 \
    --from $account \
    -b block \
    -y



