use std::ops::{Add, Mul};

pub fn calculate_termination_date(start_date_seconds: u64, duration: u64 ) -> u64 {

    let seconds_per_day: u64 =  24 * 3600;

    let termination_date = start_date_seconds.add(duration.mul(seconds_per_day));

    println!("Result of termination_date: {}", termination_date);

    return termination_date;
}

pub fn calculate_renewal_termination_date(duration: u64, termination_time: u64 ) -> u64 {

    let seconds_per_day: u64 =  24 * 3600;
    let termination_date = termination_time.add(duration.mul(seconds_per_day));

    println!("Result of renewal_termination_date: {}", termination_date);

    return termination_date;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subtract_first_and_last() {

        let result = calculate_termination_date(2400, 11);

        println!("Result of result: {}", result);
        assert_eq!(result, 952800);
    }
}
