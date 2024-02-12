#!/bin/sh

echo "We are going to create a usage policy for your contract:"

echo "Enter contract address:"
read contract

echo "Enter insurer account name:"
read account

echo "Enter insured party account name:"
read party

echo "Enter duration of a policy:"
read duration

echo "Enter deductible amount:"
read deductible

EXECUTE_CREATE_POLICY='{
  "create_policy": {
    "insurer": "'"$($CHAIN keys show -a $account)"'",
    "insured_party": "'"$($CHAIN keys show -a $party)"'",
    "duration": '"$duration"',
    "document_hash": "e0d123e5f316bef78bfdf5a008837577",
    "deductible_amount": '"$deductible"',
    "vehicle_data": [
      {
         "data_info": {
         "data_details": {
            "accelerometer": {"x": '"12"', "y": '"32"', "z": '"67"'},
            "gyroscope": {"x": '"12"', "y": '"32"', "z": '"67"'},
            "magnetometer": {"x": '"12"', "y": '"32"', "z": '"67"'},
            "location": {"lat": '"12"', "lng": '"32"'},
            "trip": "germany",
            "contract": "free",
            "vehicle_info": {
               "load_pct": '"12"',
               "temp": '"32"',
               "rpm": '"32"',
               "vss": '"44"',
               "iat": '"44"',
               "maf": '"55"',
               "throttlepo": '"4"',
               "runtm": '"8"',
               "fli": '"88"',
               "baro": '"8"',
               "load_abs": '"7"',
               "fuel_rate": '"34"',
               "odometer": '"23"'
            }
         }
         },
         "sign": "342342",
         "pubkey": "1"
      },
      {
         "data_info": {
         "data_details": {
            "accelerometer": {"x": '"12"', "y": '"32"', "z": '"67"'},
            "gyroscope": {"x": '"12"', "y": '"32"', "z": '"67"'},
            "magnetometer": {"x": '"12"', "y": '"32"', "z": '"67"'},
            "location": {"lat": '"12"', "lng": '"32"'},
            "trip": "germany",
            "contract": "free",
            "vehicle_info": {
               "load_pct": '"12"',
               "temp": '"32"',
               "rpm": '"67"',
               "vss": '"80"',
               "iat": '"44"',
               "maf": '"55"',
               "throttlepo": '"4"',
               "runtm": '"8"',
               "fli": '"88"',
               "baro": '"8"',
               "load_abs": '"7"',
               "fuel_rate": '"99"',
               "odometer": '"98"'
            }
         }
         },
         "sign": "342342",
         "pubkey": "1"
      }
    ]
  }
}
'

$CHAIN tx wasm execute $contract "$EXECUTE_CREATE_POLICY" \
    --node $NODE \
    --chain-id $CHAINID \
    --gas-prices 0.025$DENOM \
    --gas auto \
    --gas-adjustment 1.5 \
    --from $account \
    -b block \
    -y


