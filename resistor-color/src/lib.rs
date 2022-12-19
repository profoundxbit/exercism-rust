use enum_iterator::{all, Sequence};
use int_enum::IntEnum;
use std::string::ToString;
use strum_macros::Display;

#[repr(u32)]
#[derive(PartialOrd, Ord, Clone, Copy, Display, Debug, Eq, PartialEq, IntEnum, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

pub fn color_to_value(color: ResistorColor) -> u32 {
    color.int_value()
}

pub fn value_to_color_string(value: u32) -> String {
    match ResistorColor::from_int(value) {
        Ok(resistor_color) => resistor_color.to_string(),
        Err(_) => String::from("value out of range"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut colors = all::<ResistorColor>().collect::<Vec<_>>();
    colors.sort();

    colors
}
