use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::ContractError;

// Define the policy struct
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Policy {
    pub id: String,
    pub insurer: String,
    pub insured_party: String,
    pub start_time: u64,
    pub coverage: String,
    pub premium: u64,
    pub duration: u64,
    pub termination_time: u64,
    pub is_active: bool,
    pub closed: bool,
}

impl Policy {
    pub fn create(
        id: String,
        insurer: String,
        insured_party: String,
        start_time: u64,
        coverage: String,
        premium: u64,
        duration: u64,
        termination_time: u64,
        is_active: bool,
        closed: bool,
    ) -> Result<Self,ContractError> {
        
        Ok(Policy{
            id,
            insurer,
            insured_party,
            start_time,
            coverage,
            premium,
            duration,
            termination_time,
            is_active,
            closed,
        })
    }
}

