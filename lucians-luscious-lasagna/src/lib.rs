// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn expected_minutes_in_oven() -> i32 {
    40
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    let expected = expected_minutes_in_oven();
    if actual_minutes_in_oven < 0 {
        panic!("Must be in oven already!")
    } else if actual_minutes_in_oven > expected {
        panic!("Uh Oh! It's overcooked!")
    } else {
        expected - actual_minutes_in_oven
    }
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    let time_to_prepare_layer_in_minutes = 2;
    number_of_layers * time_to_prepare_layer_in_minutes
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    let prep_time = preparation_time_in_minutes(number_of_layers);

    prep_time + actual_minutes_in_oven
}
