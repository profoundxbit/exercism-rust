// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const CARS_PRODUCED_PER_HOUR: f64 = 221.0;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    f64::from(speed) * CARS_PRODUCED_PER_HOUR * production_success_rate(speed)
}

fn production_success_rate(speed: u8) -> f64 {
    match speed {
        0 => 0.0,
        1..=4 => 1.0,
        5..=8 => 0.9,
        9 | 10 => 0.77,
        _ => panic!("Invalid speed!")
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    crate::production_rate_per_hour(speed) as u32 / 60
}
