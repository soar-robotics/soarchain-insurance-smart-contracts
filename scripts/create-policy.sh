#!/bin/sh

CONTRACT_ADDRESS=$1
POLICY_ID=$2

ALLIANZE_ADDRESS=$($CHAIN keys show -a $Allianz)
BOB_ADDRESS=$($CHAIN keys show -a $Bob)


EXECUTE_CREATE_POLICY='{"create_policy":{"policy":{
    "id": "'"$POLICY_ID"'", "policy_holder":"'"$ALLIANZE_ADDRESS"'", "insured_party":"'"$BOB_ADDRESS"'", "start_date": 2400, "beneficiary":"benef", "coverage":"50", "plan":"pal", "premium":2300, "duration":12, "termination_date":0,"is_active":false, "closed":false
}, "data":[
{"data_info":{"data_details": {"accelerometer":{"x": 12, "y": 32, "z": 67}, "gyroscope":{"x": 12, "y": 32, "z": 67},"magnetometer":{"x": 12, "y": 32, "z": 67}, "location":{"lat": 12, "lng": 32}, "trip":"germany", "contract": "kjh","vehicle_info":{"load_pct": 12, "temp": 32, "rpm": 34, "vss": 20, "iat": 44, "maf": 55, "throttlepo": 4, "runtm": 8, "fli": 88, "baro": 8, "load_abs": 7, "fuel_rate": 99, "odometer": 1}}},"sign": "342342","pubkey": "1"}
,
{"data_info":{"data_details": {"accelerometer":{"x": 12, "y": 32, "z": 67}, "gyroscope":{"x": 12, "y": 32, "z": 67},"magnetometer":{"x": 12, "y": 32, "z": 67}, "location":{"lat": 12, "lng": 32}, "trip":"germany", "contract": "kjh","vehicle_info":{"load_pct": 12, "temp": 32, "rpm": 67, "vss": 80, "iat": 44, "maf": 55, "throttlepo": 4, "runtm": 8, "fli": 88, "baro": 8, "load_abs": 7, "fuel_rate": 99, "odometer": 23}}},"sign": "342342","pubkey": "1"}]}}'


$CHAIN tx wasm execute $CONTRACT_ADDRESS "$EXECUTE_CREATE_POLICY" \
    --node $NODE \
    --chain-id $CHAINID \
    --gas-prices 0.025$DENOM \
    --gas auto \
    --gas-adjustment 1.5 \
    --from $Allianz \
    -b block \
    -y

