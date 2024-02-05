use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::CustomQuery;

#[cw_serde]
#[derive(QueryResponses)]
pub enum SoarchainQuery {

    #[returns(MotusByAddressResponse)]
    MotusByAddress {
        address: String,
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
    pub pubkey: String,
    pub vin: String,
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
    pub creation_date: u64,
    pub beneficiary: String,
    pub coverage: String,
    pub plan: String,
    pub premium: u64,
    pub period: u64,
    pub closed: bool,
}