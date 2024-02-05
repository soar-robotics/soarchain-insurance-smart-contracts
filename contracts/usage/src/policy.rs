use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::ContractError;

// Define the policy struct
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Policy {
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

impl Policy {
    pub fn create(
        id: String,
        policy_holder: String,
        insured_party: String,
        creation_date: u64,
        beneficiary: String,
        coverage: String,
        plan: String,
        premium: u64,
        period: u64,
        closed: bool,
    ) -> Result<Self,ContractError> {
        
        Ok(Policy{
            id,
            policy_holder,
            insured_party,
            creation_date,
            beneficiary,
            coverage,
            plan, 
            premium,
            period,
            closed,
        })
    }
}

