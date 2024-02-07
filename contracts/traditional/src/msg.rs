use cosmwasm_schema::{cw_serde, QueryResponses};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{policy::Policy, query::Pid};

#[cw_serde]
pub struct InstantiateMsg {
    pub policy_holder: String,
    pub insured_party: String,
    pub denom: String,
    pub base_rate: u64,
    pub rate_per_mileage: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreatePolicy(Policy),
    Withdraw(WithdrawMsg),
    Renewal(RenewalMsg),
    Terminate(TerminateMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WithdrawMsg {
    pub id: String,
    pub insured_party: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RenewalMsg {
    pub id: String,
    pub premium: u64,
    pub duration: u64,
    pub coverage: String,
    pub insured_party: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TerminateMsg {
    pub id: String,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(MotusByAddressResponse)]
    MotusByAddress { address: String },

    #[returns(PaymentVerificationResponse)]
    PaymentVerification {},

    #[returns(DetailsResponse)]
    Details { id: String },
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
    pub policy_holder: String,
    pub insured_party: String,
    pub start_date: u64,
    pub beneficiary: String,
    pub coverage: String,
    pub plan: String,
    pub premium: u64,
    pub duration: u64,
    pub termination_date: u64,
    pub is_active: bool,
    pub closed: bool,
}
