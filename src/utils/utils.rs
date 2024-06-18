use crate::structs::{command::Command, flag::{Flag, FlagArg}};

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

pub fn has_flag(options: &Vec::<(&'static Flag, FlagArg)>, flag : &'static Flag) -> bool {
    return options.iter().any(|tup| tup.0.flag == flag.flag);
}

pub fn flag_u32_arg_or_default(options: &Vec::<(&'static Flag, FlagArg)>, flag : &'static Flag, default : u32) -> u32 {
    return match options.iter().find(|tup| tup.0.flag == flag.flag) {
        Some(tup) =>  match tup.1 {
            FlagArg::UnsignedInt(val) => val,
            _ => panic!(),
        }
        None => default,
    };
}

pub fn flag_u32_arg_or_none(options: &Vec::<(&'static Flag, FlagArg)>, flag : &'static Flag) -> Option<u32> {
    return match options.iter().find(|tup| tup.0.flag == flag.flag) {
        Some(tup) =>  match tup.1 {
            FlagArg::UnsignedInt(val) => Some(val),
            _ => panic!(),
        }
        None => None,
    };
}

pub fn flag_string_arg_or_default<'a>(options: &'a Vec::<(&'static Flag, FlagArg)>, flag : &'static Flag, default : &'a str) -> &'a str {
    return match options.iter().find(|tup| tup.0.flag == flag.flag) {
        Some(tup) =>  match &tup.1 {
            FlagArg::String(val) => val.as_str(),
            _ => panic!(),
        }
        None => default,
    };
}