use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::ContractError;

// Define the policy struct
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LiabilityPolicyInputs {
    pub insurer: String,
    pub insured_party: String,
    pub duration: u64,
    pub document_hash: String,
    pub liability_limit: String,
    pub vehicle_type: String,
    pub deductible_amount: u64,
    // pub terms: Terms,
    pub risk_point: RiskPoint,
    pub driving_history: DrivingHistory,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RiskPoint {
    pub age: u64,
    pub location: String,
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct Terms {
//     pub coverage: Coverage,
//     pub frequency_of_premium: String,
//     pub exclusions: Vec<String>,
//     pub usage_restrictions: String,
//     pub claims_process: String,
//     pub deductibles: Deductibles,
//     pub endorsements: Endorsements,
// }

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct Coverage {
//     pub liability: Liability,
//     pub comprehensive: Comprehensive,
//     pub collision: Collision,
// }

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct Liability {
//     pub limits_bodily_injury_per_person: u64,
//     pub limits_bodily_injury_per_accident: u64,
//     pub limits_property_damage: u64,
// }

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct Comprehensive {
//     pub limit_stolen: u64,
//     pub limit_damaged: u64,
// }

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct Collision {
//     pub limit: u64,
// }

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct Deductibles {
//     pub collision_coverage: u64,
//     pub comprehensive_coverage: u64,
// }

/// Insured party may choose to add additional coverage to their existing policy.  
// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct Endorsements{
//     pub sound_system: u64,
//     pub change_of_vehicle: u64,
//     pub increase_in_coverage_limits: u64,
//     pub change_in_usage: u64,
// }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DrivingHistory {
    pub consecutive_claim_free_years: u64, // safe driver
}

impl LiabilityPolicyInputs {
    pub fn create(
        insurer: String,
        insured_party: String,
        duration: u64,
        document_hash: String,
        liability_limit: String,
        vehicle_type: String,
        deductible_amount: u64,
        // terms: Terms,
        risk_point: RiskPoint,
        driving_history:DrivingHistory, 
    ) -> Result<Self,ContractError> {
        
        Ok(LiabilityPolicyInputs{
            insurer,
            insured_party,
            duration,
            document_hash,
            liability_limit,
            vehicle_type,
            deductible_amount,
            // terms, 
            risk_point,
            driving_history,
        })
    }
}

