use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::ContractError;

// Define the policy struct
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Policy {
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

impl Policy {
    pub fn create(
        id: String,
        policy_holder: String,
        insured_party: String,
        start_date: u64,
        beneficiary: String,
        coverage: String,
        plan: String,
        premium: u64,
        duration: u64,
        termination_date: u64,
        is_active: bool,
        closed: bool,
    ) -> Result<Self,ContractError> {
        
        Ok(Policy{
            id,
            policy_holder,
            insured_party,
            start_date,
            beneficiary,
            coverage,
            plan, 
            premium,
            duration,
            termination_date,
            is_active,
            closed,
        })
    }
}

