use std::ops::{Add, Mul};
use crate::types::Data;
use sha3::{Digest, Sha3_256};

const BASE_RATE: u64 = 1000;

pub fn calculate_avg_vss(data: &[Data]) -> u64 {
    let mut sum: u64 = 0;
    for d in data {

        sum = sum + d.data_info.data_details.vehicle_info.vss

    }
    let avg_vss = sum / data.len() as u64;
    return avg_vss;
}

pub fn calculate_avg_rpm(data: &[Data]) -> u64 {
    let mut sum: u64 = 0;
    for d in data {

        sum = sum + d.data_info.data_details.vehicle_info.rpm

    }
    let avg_rpm = sum / data.len() as u64;
    return avg_rpm;
}

/// Creates a unique policy ID based on the insurer, insured party, and start date.
///
/// # Arguments
///
/// * `insurer` - The identifier of the insurance company.
/// * `insured_party` - The identifier of the insured party.
/// * `start_time` - The start time of the policy in seconds since the epoch.
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

pub fn calculate_termination_time(start_time_seconds: u64, duration: u64 ) -> u64 {

    let seconds_per_day: u64 =  24 * 3600;

    let termination_time = start_time_seconds.add(duration.mul(seconds_per_day));

    log::info!("Result of renewal_termination_time: {}", termination_time);

    return termination_time;
}

pub fn calculate_renewal_termination_time(duration: u64, termination_time: u64 ) -> u64 {

    let seconds_per_day: u64 =  24 * 3600;
    let termination_time = termination_time.add(duration.mul(seconds_per_day));

    log::info!("Result of renewal_termination_time: {}", termination_time);

    return termination_time;
}

pub fn calculate_premium(data: &[Data]) -> u64 {

    let avg_vss = calculate_avg_vss(data);
    let avg_rpm = calculate_avg_rpm(data);

    if avg_vss < 80 &&  avg_rpm < 2500 {
        return BASE_RATE / 2
    } 
    return BASE_RATE;
}

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