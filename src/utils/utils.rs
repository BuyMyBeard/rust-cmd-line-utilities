use crate::structs::command::Command;

use super::errors::terminate_invalid_flag_error;

pub enum ArgType {
    None,
    Argument(String),
    Flag(String),
}

trait OptionArgType {
    fn get_arg_type(&self, next : &Self, command : Command);
}

impl OptionArgType for Option<String> {
    /// Transform an Option<String> returned by iterator.peek() or iterator.next()
    fn get_arg_type(&self, next : &Self, command : Command) {
        let arg = &self.clone().unwrap();
        let first_char = arg.chars().nth(0).unwrap();
        
        if first_char == '-' {
            let flag_arg = &arg[1..arg.len()];
            let flag = match command.options.iter().find(|flag: &&&crate::structs::flag::Flag| flag.flag.to_lowercase() == flag_arg) {
                Some(flag) => flag,
                None => terminate_invalid_flag_error(flag_arg, command.name),
            };
        }  
    }
}

pub trait IsFlag {
    fn is_flag(&self) -> Option<String>;
}

/// Returns Some() without the prefix or None if not a flag
impl IsFlag for Option<String> {
    fn is_flag(&self) -> Option<String> {
        let string = match &self {
            Some(value) => value,
            None => return None,
        };
        let first_char = match string.chars().nth(0) {
            Some(value) => value,
            None => return None,
        };
        return match first_char {
            '-' => Some(String::from(&string[1..string.len()])),
            _ => None,
        };
    }
}

pub fn is_flag(arg: &str) -> bool {
    let first_char = match arg.trim().chars().nth(0) {
        Some(value) => value,
        None => return false,
    };
    return match first_char {
        '-' => true,
        _ => false,
    };
}