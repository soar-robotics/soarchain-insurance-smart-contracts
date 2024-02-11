use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::ContractError;

// Define the policy struct
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LiabilityPolicy {
    pub id: String,
    pub vehicle: Vehicle,
    pub insurance_type: String,
    /// The address of the policy owner
    pub insurer: String,
    /// The address of who purchases the insurance policy and is protected by its terms
    pub insured_party: String,
    /// `document_hash` represents a binary hash uniquely identifying documents associated with this insurance policy.
    /// Before assigning a value to this property, create a binary hash for your documents and encode it as a base64 string.
    /// You can use the provided repository tools to generate the binary hash for your documents. // https://github.com/dadadel/binmake
    /// It is crucial to ensure the integrity and security of the documents linked to this insurance policy.
    pub document_hash: String,
    /// The policy start date
    pub start_time: u64,
    /// `terms` holds a JSON object representing the detailed terms and conditions associated with this insurance policy.
    /// When creating a new policy, populate this property with a JSON object encapsulating all relevant terms.
    /// Ensure to encode the JSON object as a string before setting it to this property.
    /// It's recommended to use a robust JSON parser to decode and interpret the policy terms when needed.
    pub terms: Terms,
    pub risk_point: RiskPoint,
    pub premium: u64,
    pub base_rate: u64,
    pub duration: u64,
    pub termination_time: u64,
    pub is_active: bool,
    pub closed: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Vehicle {
    pub vin: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RiskPoint {
    pub location: String,
    pub age: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Terms {
    pub coverage: Coverage,
    // pub frequency_of_premium: String,
    pub exclusions: String,
    // pub usage_restrictions: String,
    pub claims_process: String,
    // pub deductibles: Deductibles,
    // pub endorsements: Endorsements,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Coverage {
    pub liability: Liability,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Liability {
    pub limits_bodily_injury_per_person: u64,
    pub limits_bodily_injury_per_accident: u64,
    pub limits_property_damage: u64,
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct Comprehensive {
//     pub limit_stolen: u64,
//     pub limit_damaged: u64,
// }

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct Collision {
//     pub limit: u64,
// }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Deductibles {
    pub collision_coverage: u64,
}

/// Insured party may choose to add additional coverage to their existing policy.  
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Endorsements{
    pub sound_system: u64,
    pub change_of_vehicle: u64,
    pub increase_in_coverage_limits: u64,
    pub change_in_usage: u64,
}

impl LiabilityPolicy {
    pub fn create(
        id: String,
        vehicle: Vehicle,
        insurance_type: String,
        insurer: String,
        insured_party: String,
        document_hash: String, 
        start_time: u64,
        terms: Terms,
        risk_point: RiskPoint,
        premium: u64,
        base_rate: u64,
        duration: u64,
        termination_time: u64,
        is_active: bool,
        closed: bool,
    ) -> Result<Self,ContractError> {
        
        Ok(LiabilityPolicy{
            id,
            vehicle,
            insurance_type,
            insurer,
            insured_party,
            document_hash,
            start_time,
            terms, 
            risk_point,
            premium,
            duration,
            base_rate,
            termination_time,
            is_active,
            closed,
        })
    }

}

