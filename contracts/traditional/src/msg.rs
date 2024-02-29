use cosmwasm_schema::{cw_serde, QueryResponses};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{inputs::LiabilityPolicyInputs, liabilitypolicy::{RiskPoint, Terms, Vehicle}, query::Pid};

#[cw_serde]
pub struct InstantiateMsg {
    pub denom: String,
    pub insurer: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreateLiabilityPolicy(LiabilityPolicyInputs),
    Withdraw(WithdrawMsg),
    Renewal(RenewalMsg),
    Terminate(TerminateMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WithdrawMsg {
    pub insured_party: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RenewalMsg {
    pub premium: u64,
    pub duration: u64,
    pub insured_party: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TerminateMsg {
    pub insured_party: String,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(MotusByAddressResponse)]
    MotusByAddress { address: String },

    #[returns(PaymentVerificationResponse)]
    PaymentVerification { id: String },

    #[returns(DetailsResponse)]
    Details { address: String },

    #[returns(ListResponse)]
    List {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct MotusByAddressResponse {
    pub address: String,
    pub pubkey: String,
    pub vin: String,
    pub pid: Pid
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct PaymentVerificationResponse {
    pub verified: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct DetailsResponse {
    pub id: String,
    pub vehicle: Vehicle,
    pub insurance_type: String,
    pub insurer: String,
    pub insured_party: String,
    pub document_hash: String,
    pub start_time: String,
    pub terms: Terms,
    pub risk_point: RiskPoint,
    pub premium: u64,
    pub duration: u64,
    pub termination_time: String,
    pub is_active: bool,
    pub closed: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ListResponse {
    /// list all registered vehicle owners
    pub insured_parties: Vec<String>,
}
