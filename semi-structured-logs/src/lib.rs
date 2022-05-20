// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    match level {
        LogLevel::Info => info(message),
        LogLevel::Warning => warn(message),
        LogLevel::Error => error(message)
    }
}
pub fn info(message: &str) -> String {
    let mut info_msg = "[INFO]: ".to_owned();
    info_msg += message;
    // let mut message = message.to_string();
    // message.insert_str(0, "[INFO]: ");
    info_msg
}
pub fn warn(message: &str) -> String {
    let mut message = message.to_string();
    message.insert_str(0, "[WARNING]: ");
    message
}
pub fn error(message: &str) -> String {
    let mut message = message.to_string();
    message.insert_str(0, "[ERROR]: ");
    message
}
