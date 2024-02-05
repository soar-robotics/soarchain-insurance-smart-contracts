
use crate::types::Data;

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

pub fn calculate_mileage(data: &[Data]) -> u64 {
    let mut result: u64 = 0;
    if let (Some(first), Some(last)) = (data.first(), data.last()) {
        result = (first.data_info.data_details.vehicle_info.odometer as u64)
            - (last.data_info.data_details.vehicle_info.odometer as u64);

        println!("Result of subtraction: {}", result);
    }
    return result;
}


// fn subtract_first_and_last(vec_u64: &Vec<u64>) -> Result<.round() as u64, &'static str> {
//     // Check if the Vec has at least two elements
    
//     if vec_u64.len() >= 2 {
//         // Subtract the last element from the first element and convert to u64
//         let result =  (vec_u64[vec_u64.len() - 1] as u64) - (vec_u64[0] as u64);
//         println!("Result of subtraction: {}", result);
//         Ok(result)
//     } else {
//         Err("Vec<u64> must have at least two elements")
//     }
// }

// fn subtract_first_and_last() -> Result<u64, ContractError> {
//     // Example Vec<u64>
//     let vec_u64: Vec<u64> = vec![10, 5, 2, 3];

//     // Check if the Vec is not empty
//     if let (Some(first), Some(last)) = (vec_u64.first(), vec_u64.last()) {
//         // Multiply the values and convert to u64
//         let result = (*first as u64) - (*last as u64);

//         // Print the result
//         println!("Result of multiplication: {}", result);
//         Ok(result)
//     } else {
//         return Err(ContractError::NoData {})
//     }
// }


// #[cfg(test)]
// mod tests {
//     use super::*;


//     #[test]
//     fn test_subtract_first_and_last() {
//         // Test case with valid Vec<u64>
//         let result = subtract_first_and_last();
//         assert_eq!(result, Ok(7.0));

//         // Test case with Vec<u64> having less than two elements
//         // let result = subtract_first_and_last();
//         // assert_eq!(result, Err("Vec<u64> must have at least two elements"));
//     }
// }


