use std::collections::HashMap;
use std::ops::Sub;
use std::ops::{Add, Mul};
use sha3::{Digest, Sha3_256};
use crate::inputs::RiskPoint;
use crate::inputs::DrivingHistory;
use crate::constants::LIABILITY_BASE_RATE;//LIABILITY_BASE_RISK

const MAX_DISCOUNT: u64 = 15 / 100; // Maximum discount percentage

// Define the discount increment for each consecutive claim-free year
const DISCOUNT_INCREMENT: u64 =  2 / 100; // 2% discount per year

/// Calculates the termination date based on the start date and policy duration.
///
/// # Arguments
///
/// * `start_date_seconds` - The start date of the policy in seconds since the epoch.
/// * `duration` - The duration of the policy in days.
///
/// ```
pub fn calculate_termination_time(start_time_seconds: u64, duration: u64 ) -> u64 {

    let seconds_per_day: u64 =  24 * 3600;

    let termination_time = start_time_seconds.add(duration.mul(seconds_per_day));

    log::info!("Result of renewal_termination_time: {}", termination_time);

    return termination_time;
}

/// Calculates the termination date for policy renewal based on the existing termination time and duration.
///
/// # Arguments
///
/// * `duration` - The duration of the renewed policy in days.
/// * `termination_time` - The existing termination time of the policy in seconds since the epoch.
///
/// ```
pub fn calculate_renewal_termination_time(duration: u64, termination_time: u64 ) -> u64 {

    let seconds_per_day: u64 =  24 * 3600;
    let termination_time = termination_time.add(duration.mul(seconds_per_day));

    log::info!("Result of renewal_termination_time: {}", termination_time);

    return termination_time;
}



pub fn split_and_convert(input: &str) -> (u64, u64, u64) {
    let limits: Vec<&str> = input.split('/').collect();
    
    let limit1 = limits[0].parse().unwrap_or(0);
    let limit2 = limits[1].parse().unwrap_or(0);
    let limit3 = limits[2].parse().unwrap_or(0);

    (limit1, limit2, limit3)
}

/// Creates a unique policy ID based on the insurer, insured party, and start date.
///
/// # Arguments
///
/// * `insurer` - The identifier of the insurance company.
/// * `insured_party` - The identifier of the insured party.
/// * `start_date` - The start date of the policy in seconds since the epoch.
/// ```
pub fn create_policy_id(insurer: &str, insured_party: &str, start_date: u64) -> String {
    // Combine relevant fields to create a unique input for hashing
    let input_data = format!("{}{}{}", insurer, insured_party, start_date);

    // Use SHA-3 (Keccak) hash function to create a deterministic hash
    let mut hasher = Sha3_256::new();
    hasher.update(input_data.as_bytes());

    // Convert the hash result to a hexadecimal string
    let hash_result = hasher.finalize();
    format!("{:x}", hash_result)
}

// Function to check if a policy has terminated
pub fn is_policy_terminated(termination_time_seconds: u64, current_time_seconds: u64) -> bool {

    if current_time_seconds >= termination_time_seconds {
        return true;
    }
    return false;    
}

// Function to check if a policy is eligible for renewal
pub fn is_policy_eligible_for_renewal(
    current_time_seconds: u64,
    renewal_time_seconds: u64,
    termination_time_seconds: u64,
) -> bool {

    let renewal_time = current_time_seconds.add(renewal_time_seconds);
    // Check if the current time is within the renewal window before termination

    if renewal_time  >= termination_time_seconds {
        return true;
    }
    return false;    
}

fn calculate_age_factor(age: u64) -> u64 {
    // Define age ranges and corresponding factors in a HashMap
    let mut factor_ranges = HashMap::new();
    factor_ranges.insert(0..=25, 1);
    factor_ranges.insert(26..=40, 1);
    factor_ranges.insert(41..=60, 1);
    factor_ranges.insert(61..=80, 2);
    // Add more ranges and factors as needed

    // Find the matching factor based on the age
    let factor = factor_ranges
        .iter()
        .filter(|(range, _)| range.contains(&age))
        .map(|(_, factor)| *factor)
        .next()
        .unwrap_or(1);

    // Return the calculated factor
    factor
}

pub fn calculate_age_risk_amount(age: u64) -> u64 {

    let age_risk_amount = LIABILITY_BASE_RATE * calculate_age_factor(age);

    age_risk_amount
}

#[derive(Debug, Eq, PartialEq, Hash)]
// #[allow(dead_code)]  // Allow dead code for unused enum variants  // Add Eq, PartialEq, and Hash traits
enum VehicleType {
    Sedan,
    SUV,
    Truck,
    Motorcycle,
    // Add more vehicle types as needed
}

impl VehicleType {
    // Function to convert a string to VehicleType
    fn from_str(s: &str) -> VehicleType {
        match s.to_lowercase().as_str() {
            "sedan" => VehicleType::Sedan,
            "suv" => VehicleType::SUV,
            "truck" => VehicleType::Truck,
            "motorcycle" => VehicleType::Motorcycle,
            _ => VehicleType::Sedan, // Default to Sedan if the conversion fails
        }
    }
}

fn calculate_vehicle_type_factor(vehicle_type_str: &str) -> u64 {
    // Convert the input string to VehicleType
    let vehicle_type = VehicleType::from_str(vehicle_type_str);

    // Define vehicle type factors in a HashMap
    let mut factor_map = HashMap::new();
    factor_map.insert(VehicleType::Sedan, 1);
    factor_map.insert(VehicleType::SUV, 1);
    factor_map.insert(VehicleType::Truck, 1);
    factor_map.insert(VehicleType::Motorcycle, 2);
    // Add more vehicle types and factors as needed

    // Find the matching factor based on the vehicle type
    let factor = factor_map.get(&vehicle_type).copied().unwrap_or(1);

    // Return the calculated factor
    factor
}

pub fn calculate_vehicle_type_risk_amount(vehicle_type: String) -> u64 {

    let vehicle_type_amount = LIABILITY_BASE_RATE.mul(calculate_vehicle_type_factor(&vehicle_type));

    vehicle_type_amount
}

// Function to calculate the Safe Driver Discount Factor
pub fn calculate_safe_driver_discount_factor(driving_history: DrivingHistory) -> u64 {


    // Calculate the discount factor based on consecutive claim-free years
    let discount_factor = driving_history.consecutive_claim_free_years.mul(DISCOUNT_INCREMENT);

    // Ensure the discount factor does not exceed the maximum discount
    discount_factor.min(MAX_DISCOUNT)
}

pub fn calculate_safe_driver_discount(driving_history: DrivingHistory) -> u64 {

    // Calculate the discount factor based on consecutive claim-free years
    let discount = LIABILITY_BASE_RATE.mul(calculate_safe_driver_discount_factor(driving_history));

    discount
}

fn calculate_liability_factor(liability_limit: String) -> u64 {
    // Assume that liability_limit is a string like "100/300/50"
    let limits: Vec<u64> = liability_limit
        .split('/')
        .map(|limit| limit.parse().unwrap_or(0))
        .collect();

    // Extract individual limits
    let bodily_injury_per_person_limit = limits.get(0).unwrap_or(&0);
    let bodily_injury_per_accident_limit = limits.get(1).unwrap_or(&0);
    let property_damage_limit = limits.get(2).unwrap_or(&0);

    // Example calculation (you might need a more sophisticated logic)
    let total_limit = *bodily_injury_per_person_limit + *bodily_injury_per_accident_limit + *property_damage_limit;

    // Define ranges and corresponding factors in a HashMap
    let mut factor_ranges = HashMap::new();
    factor_ranges.insert(0..=100, 1);
    factor_ranges.insert(101..=200, 2);
    factor_ranges.insert(201..=300, 3);
    factor_ranges.insert(301..=400, 4);
    factor_ranges.insert(401..=500, 5);
    // Add more ranges and factors as needed

    // Find the matching factor based on the total limit
    let factor = factor_ranges
        .iter()
        .find(|(range, _)| range.contains(&total_limit))
        .map(|(_, factor)| *factor)
        .unwrap_or(4);

    // Return the calculated factor
    factor
}

// Function to calculate the Safe Driver Discount Factor
pub fn calculate_limit_based_amount(liability_limit: String) -> u64 {

    let liability_amount = LIABILITY_BASE_RATE.mul(calculate_liability_factor(liability_limit));
    liability_amount
}

fn calculate_deductible_factor(deductible_amount: u64) -> u64 {
    // Example logic for calculating deductible factor
    return match deductible_amount {
        0..=500 => 1,
        501..=1000 => 1,
        1001..=1500 => 1,
        _ => 2,
    }
}

pub fn calculate_deductible_amount(deductible_amount: u64) -> u64 {

    let deductible_amount = LIABILITY_BASE_RATE.mul(calculate_deductible_factor(deductible_amount));
    deductible_amount
}

pub fn calculate_premium(risk_point: RiskPoint, driving_history: DrivingHistory, vehicle_type: String, liability_limit: String, deductible_amount: u64) -> u64 {
    let result = LIABILITY_BASE_RATE
    .add(calculate_age_risk_amount(risk_point.age))
    .add(calculate_vehicle_type_risk_amount(vehicle_type))
    .add(calculate_limit_based_amount(liability_limit))
    .sub(calculate_safe_driver_discount(driving_history))
    .sub(calculate_deductible_amount(deductible_amount));
    return result
}

// pub fn calculate_coverage_amount(
//     insured_amount: u64,
// ) -> u64 {

//     return insured_amount.mul(calculate_coverage_rate);
// }

// #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
// enum RiskFactors {
//     Age,
//     HealthHistory,
//     Occupation,
//     LifestyleHabits,
//     CoverageType,
//     GeographicalLocation,
//     ClaimHistory,
//     Gender,
//     CreditHistory,
//     Location,
// }

// Implement the Iterator trait for the RiskFactors enum
// impl RiskFactors {
//     fn iter() -> impl Iterator<Item = &'static RiskFactors> {
//         static RISK_FACTORS: [RiskFactors; 10] = [
//             RiskFactors::Age,
//             RiskFactors::HealthHistory,
//             RiskFactors::Occupation,
//             RiskFactors::LifestyleHabits,
//             RiskFactors::CoverageType,
//             RiskFactors::GeographicalLocation,
//             RiskFactors::ClaimHistory,
//             RiskFactors::Gender,
//             RiskFactors::CreditHistory,
//             RiskFactors::Location,
//         ];
//         RISK_FACTORS.iter()
//     }
// }

// pub fn calculate_coverage_factor(risk_points: RiskPoint, driving_history: DrivingHistory, premium: u64) -> u64 {

//     //return 1.add(calculate_risk_factor(risk_points, premium).sub(calculate_safe_driver_discount(driving_history)));
//     return 1
// }


// fn calculate_risk_amount(risk_points: RiskPoint, premium: u64) -> u64 {

//     let mut risk_factor_amount = 0;

//     // Enumerate through the risk factors
//     for risk_factors in RiskFactors::iter() {
//         match risk_factors {
//             RiskFactors::Age => {

//                 if risk_points.age >= 65 {
//                     let ten_percent_of_premium = (premium * 10) / 100;
//                     risk_factor_amount = LIABILITY_BASE_RISK.add(ten_percent_of_premium);
//                 }
//             }
//             RiskFactors::Location => {
//                 if risk_points.location.eq("risky") {
//                     let two_percent_of_premium = (premium * 2) / 100;
//                     risk_factor_amount = risk_factor_amount.add(two_percent_of_premium);
//                 }
//             }
//             /* 
//              * implementation ...
//              * Your logic goes here
//              * Add more cases for other risk factors as needed
//             */
//             _ => {}
//         }
//     }

//     return risk_factor_amount;
// }




#[cfg(test)]
mod tests {
    use cosmwasm_std::Decimal256;

    use super::*;
    
    #[test]
    fn test_traditional_policy_process() {

        let termination_time = calculate_termination_time(2400, 11);
        println!("Result of termination_time: {}", termination_time);
        assert_eq!(termination_time,952800u64);

        let policy_id = create_policy_id("insurer", "insured_party", 2400);
        println!("Result of policy_id: {}", policy_id);
        assert_eq!(policy_id, "5f2eac0d262b66e1af329362e32b29fc481351ae6be4387a905310dd2d4f276d");

        let policy_terminated: bool = is_policy_terminated(2000, 4000);
        println!("Result of policy_terminated: {}", policy_terminated);
        assert_eq!(policy_terminated, true);

        let policy_eligibility = is_policy_eligible_for_renewal(1000, 1000, 1400);
        println!("Result of policy_eligibility: {}", policy_eligibility);
        assert_eq!(policy_eligibility, true);

        // let  risk_points = RiskPoint { age: 65, location: "riskey".to_owned() /* other fields here */ };

        // let calculate_risk_factor = calculate_risk_factor(risk_points, 1000);
        // println!("Result of calculate_risk_factor: {}", calculate_risk_factor);
        // assert_eq!(calculate_risk_factor, 180);

        // let  risk_points = RiskPoint { age: 65, location: "riskey".to_owned() /* other fields here */ };
        // let  driving_history = DrivingHistory { consecutive_claim_free_years: 3 /* other fields here */ };


        // let calculate_coverage_factor = calculate_coverage_factor(risk_points, driving_history,1000);
        // println!("Result of calculate_coverage_factor: {}", calculate_coverage_factor);
        //assert_eq!(calculate_risk_factor, 181);

    }

    #[test]
    fn test_liability_policy_factors() {

        let liability_limit1 = "100/300/50";
        let liability_limit2 = "100/10/15";
    
        let factor1 = calculate_liability_factor(liability_limit1.to_string());
        let factor2 = calculate_liability_factor(liability_limit2.to_string());
    
        // Display the results
        println!("Liability Factor 1: {}", factor1);
        println!("Liability Factor 2: {}", factor2);

    }

    #[test]
    fn test_calculate_age_factors() {

        let age1 = 22;
        let age2 = 35;
        let age3 = 50;
    
        let factor1 = calculate_age_factor(age1);
        let factor2 = calculate_age_factor(age2);
        let factor3 = calculate_age_factor(age3);
    
        // Display the results
        println!("Age Factor 1: {}", factor1);
        println!("Age Factor 2: {}", factor2);
        println!("Age Factor 3: {}", factor3);

    }

    #[test]
    fn test_calculate_vehicle_type_factor() {

        // Example factor calculation with different vehicle types
        let factor_sedan = calculate_vehicle_type_factor("sedan");
        let factor_suv = calculate_vehicle_type_factor("suv");
        let factor_truck = calculate_vehicle_type_factor("truck");
        let factor_motorcycle = calculate_vehicle_type_factor("motorcycle");

        // Display the results
        println!("Factor for Sedan: {}", factor_sedan);
        println!("Factor for SUV: {}", factor_suv);
        println!("Factor for Truck: {}", factor_truck);
        println!("Factor for Motorcycle: {}", factor_motorcycle);

    }

    #[test]
    fn test_parse_liability_limit() {

        // Example usage
        let input_string = "100/300/50";
        let (limit1, limit2, limit3) = split_and_convert(input_string);

        // Display the results
        println!("First Limit: {}", limit1);
        println!("Second Limit: {}", limit2);
        println!("Third Limit: {}", limit3);

     }

    #[test]
    fn test_decimal() {

        let decimal = Decimal256::from_atomics(12u64, 1).unwrap();
        println!("decimal---->: {}", decimal);
        //assert_eq!(decimal.to_string(),Decimal::from() "1.2");

    }

}


