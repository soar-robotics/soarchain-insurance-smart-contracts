use cosmwasm_schema::{cw_serde, QueryResponses};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::policy::Policy;
use crate::query::Pid;
use crate::types::Data;

#[cw_serde]
pub struct InstantiateMsg {
    pub denom: String,
    pub insurer: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreateMileagePolicy(CreateMsg),
    Withdraw(WithdrawMsg),
    Renewal(RenewalMsg),
    Terminate(TerminateMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CreateMsg {
    pub insurer: String,
    pub insured_party: String,
    pub duration: u64,
    pub document_hash: String,
    pub deductible_amount: u64,
    pub vehicle_data: Vec<Data>
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InsurancePolicyData {
    pub policy: Policy,
    pub data: Vec<Data>,
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
    pub insurer: String,
    pub insured_party: String,
    pub start_time: u64,
    pub premium: u64,
    pub duration: u64,
    pub termination_time: u64,
    pub is_active: bool,
    pub closed: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ListResponse {
    /// list all registered vehicle owners
    pub insured_parties: Vec<String>,
}