#!/bin/sh

CONTRACT_ADDRESS=$1
INSURED_PARTY=$2

EXECUTE_CREATE_POLICY='{"create_liability_policy":{
     "insurer":"'"$($CHAIN keys show -a $Allianz)"'",
     "insured_party":"'"$($CHAIN keys show -a $INSURED_PARTY)"'", 
     "duration":'"3"', 
     "document_hash":"e0d123e5f316bef78bfdf5a008837577", 
     "liability_limit":"100/300/50",
     "vehicle_type":"sedan",
     "deductible_amount": '"500"',
     "risk_point":{"age": '"45"', "location":"paris"},
     "driving_history":{"consecutive_claim_free_years": '"3"'}
  }
}';

#2EXECUTE_CREATE_POLICY='{"create_policy":{"id": "'"$POLICY_ID"'","vehicle":{"vin":"jgjhgj"}, "insurer":"'"$($CHAIN keys show -a $Allianz)"'", "insured_party":"'"$($CHAIN keys show -a $INSURED_PARTY)"'","document_hash":"hjgjhg", "start_time":'"2200"', "terms":{"coverage":{"liability":{"limits_bodily_injury_per_person":'"4555"',"limits_bodily_injury_per_accident":'"888"',"limits_property_damage":'"67567"'},"comprehensive":{"limit_stolen":'"89"',"limit_damaged":'"234"'},"collision":{"limit":'"456"'}},"frequency_of_premium":"daily","exclusions":["racing"],"usage_restrictions":"gfhfgh","claims_process":"fghfgh","deductibles":{"collision_coverage":'"78678"',"comprehensive_coverage":'"678678"'},"endorsements":{"sound_system":'"4354"',"change_of_vehicle":'"456456"',"increase_in_coverage_limits":'"56564"',"change_in_usage":'"4564"'},"policy_period":{"from":"gjghjg","to":"ghjghj"}},"risk_point":{"location":"paris", "driving_record":"80", "age":'"40"'}, "premium":'"2000"', "duration":'"3"', "termination_time":'"0"',"is_active":'"false"',"closed":'"false"'}}'

# EXECUTE_CREATE_POLICY='{"create_policy":{"id": "'"$POLICY_ID"'","vehicle":{"vin":"jgjhgj"}, "insurer":"'"$($CHAIN keys show -a $Allianz)"'", "insured_party":"'"$($CHAIN keys show -a $INSURED_PARTY)"'","document_hash":"hjgjhg", "start_time":'"2200"', "terms":{"coverage":{"liability":{"limits_bodily_injury_per_person":'"4555"',"limits_bodily_injury_per_accident":'"888"',"limits_property_damage":'"67567"'},"comprehensive":{"limit_stolen":'"89"',"limit_damaged":'"234"'},"collision":{"limit":'"456"'}},"frequency_of_premium":"daily","exclusions":["racing"],"usage_restrictions":"gfhfgh","claims_process":"fghfgh","deductibles":{"collision_coverage":'"78678"',"comprehensive_coverage":'"678678"'},"endorsements":{"sound_system":'"4354"',"change_of_vehicle":'"456456"',"increase_in_coverage_limits":'"56564"',"change_in_usage":'"4564"'},"policy_period":{"from":"gjghjg","to":"ghjghj"}},"risk_point":{"location":"paris", "driving_record":"80", "age":'"40"'}, "premium":'"2000"', "duration":'"3"', "termination_time":'"0"',"is_active":'"false"',"closed":'"false"'}}'
$CHAIN tx wasm execute $CONTRACT_ADDRESS "$EXECUTE_CREATE_POLICY" \
    --node $NODE \
    --chain-id $CHAINID \
    --gas-prices 0.025$DENOM \
    --gas auto \
    --gas-adjustment 1.5 \
    --from $Allianz \
    -b block \
    -y



