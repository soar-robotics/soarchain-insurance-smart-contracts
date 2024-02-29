
use std::ops::{Add, Mul};
use sha3::{Digest, Sha3_256};
use crate::types::Data;

pub fn calculate_mileage(data: &[Data]) -> u64 {

    let mut mileage: u64 = 0;

    if let (Some(first), Some(last)) = (data.first(), data.last()) {
        let first: u64 = first.data_info.data_details.vehicle_info.odometer;
        let last: u64 = last.data_info.data_details.vehicle_info.odometer;
        mileage = last - first;
      }
    return mileage;
}

pub fn calculate_renewal_termination_date(duration: u64, termination_time: u64 ) -> u64 {

    let seconds_per_day: u64 =  24 * 3600;
    let termination_date = termination_time.add(duration.mul(seconds_per_day));

    println!("Result of renewal_termination_date: {}", termination_date);

    return termination_date;
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subtract_first_and_last() {

        let result = calculate_termination_time(2400, 11);

        println!("Result of result: {}", result);
        assert_eq!(result, 952800);
    }

}

