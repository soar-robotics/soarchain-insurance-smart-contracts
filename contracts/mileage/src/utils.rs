
use crate::types::Data;

pub fn calculate_mileage(data: &[Data]) -> u64 {
    let mut result: u64 = 0;
    if let (Some(first), Some(last)) = (data.first(), data.last()) {
        result = (first.data_info.data_details.vehicle_info.odometer as u64)
            - (last.data_info.data_details.vehicle_info.odometer as u64);

        println!("Result of subtraction: {}", result);
    }
    return result;
}
