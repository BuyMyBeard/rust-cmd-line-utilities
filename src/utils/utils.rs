use crate::structs::flag::{Flag, FlagArg};

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