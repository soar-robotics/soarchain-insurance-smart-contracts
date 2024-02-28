use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct Data {
    pub data_info: DataInfo,
    pub sign: String,
    pub pubkey: String
}

#[cw_serde]
pub struct DataInfo {
    pub data_details: DataDetails
}

#[cw_serde]
pub struct DataDetails {
    pub accelerometer: GeographicInfo, 
    pub gyroscope: GeographicInfo,
    pub magnetometer: GeographicInfo,
    pub location: LocationInfo,
    pub trip: String,
    pub contract: String, 
    pub vehicle_info: VehicleInfo
}

#[cw_serde]
pub struct GeographicInfo {
    pub x: u64,
    pub y: u64,
    pub z: u64
}

#[cw_serde]
pub struct LocationInfo {
    pub lat: u64,
    pub lng: u64
}

#[cw_serde]
pub struct VehicleInfo {
    pub load_pct: u64,
    pub temp: u64,
    pub rpm: u64,
    pub vss: u64,
    pub iat:u64,
    pub maf: u64, 
    pub throttlepo: u64,
    pub runtm: u64, 
    pub fli: u64,
    pub baro: u64,
    pub load_abs: u64,
    pub fuel_rate: u64,
    pub odometer: u64
}

#[cw_serde]
pub struct VinInfo {
    pub region: String,
    pub car_type: String,
    pub make: String,
    pub manufacture: String,
    pub model:String,
    pub model_year: String, 
    pub body_style: String,
    pub series: u64,
    pub cylinders: u64,
    pub engin_model: String,
    pub engin_break: u64,
    pub production_number: u64,
    pub fuel_supply_system: String,
    pub driving_type: String
}
