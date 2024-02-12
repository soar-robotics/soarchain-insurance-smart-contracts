use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{error::ContractError, types::Data};

// Define the policy inputs struct
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PolicyInputs {
    pub insurer: String,
    pub insured_party: String,
    pub duration: u64,
    pub document_hash: String,
    pub deductible_amount: u64,
    pub vehicle_data: Vec<Data>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RiskPoint {
    pub age: u64,
    pub location: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DrivingHistory {
    pub consecutive_claim_free_years: u64, // safe driver
}

impl PolicyInputs {
    pub fn create(
        insurer: String,
        insured_party: String,
        duration: u64,
        document_hash: String,
        deductible_amount: u64,
        vehicle_data: Vec<Data>,
    ) -> Result<Self,ContractError> {
        
        Ok(PolicyInputs{
            insurer,
            insured_party,
            duration,
            document_hash,
            deductible_amount,
            vehicle_data,
        })
    }
}

