use std::ops::{Add, Mul};
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

pub fn calculate_renewal_termination_date(duration: u64, termination_time: u64 ) -> u64 {

    let seconds_per_day: u64 =  24 * 3600;
    let termination_date = termination_time.add(duration.mul(seconds_per_day));

    println!("Result of renewal_termination_date: {}", termination_date);

    return termination_date;
}