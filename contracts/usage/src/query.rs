use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::CustomQuery;

#[cw_serde]
#[derive(QueryResponses)]
pub enum SoarchainQuery {

    #[returns(MotusByAddressResponse)]
    MotusByAddress {
        address: String,
        dpr: String,
    },

    #[returns(PaymentVerificationResponse)]
    PaymentVerification {},
}

impl CustomQuery for SoarchainQuery {}

impl SoarchainQuery {
}

#[cw_serde]
pub struct MotusByAddressResponse {
    pub address: String,
    pub dpr_id: String,
    pub pubkey: String,
    pub vin: String,
    pub dpr: String,
    pub pid: Pid,
}

#[cw_serde]
#[serde(rename_all = "snake_case")]
pub struct Pid {
	pub pid_1_to_20: String,
	pub pid_21_to_40: String,
	pub pid_41_to_60: String,
	pub pid_61_to_80: String,
	pub pid_81_to_a0: String,
	pub pid_a1_to_c0: String,
	pub pid_c1_to_e0: String,
	pub pid_svc_to_9: String
}

#[cw_serde]
pub struct PaymentVerificationResponse {
    pub verified: bool,
}

#[cw_serde]
pub struct DetailsResponse {
    pub id: String,
    pub policy_holder: String,
    pub insured_party: String,
    pub start_time: u64,
    pub premium: u64,
    pub duration: u64,
    pub is_active: bool,
    pub closed: bool,
}