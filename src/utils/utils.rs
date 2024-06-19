use std::{net::{IpAddr, Ipv4Addr, Ipv6Addr}, num::ParseIntError};

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

pub fn try_parse_string_to_ip(string : &str) -> Option<IpAddr> {
    let ipv4_split : Vec<&str> = string.split(|c| c == '.').collect();
    if ipv4_split.len() == 4 {
        return try_parse_to_ipv4(ipv4_split);
    }
    let mut ipv6_split : Vec<&str> = string.split(|c| c == ':').collect();
    if ipv6_split.iter().filter(|substr| **substr == "").count() > 1 {
        return None;
    }
    if let Some(index) = ipv6_split.iter().position(|substr| **substr == *"") {
        let byte_count = ipv6_split.len();
        if byte_count > 8 {
            return None;
        }
        ipv6_split[index] = "0";
        for _ in 0..8 - byte_count {
            ipv6_split.insert(index, "0");
        }
    }
    if ipv6_split.len() == 8 {
        return try_parse_to_ipv6(ipv6_split);
    }
    None
}

/// Expects a vector of length 4
fn try_parse_to_ipv4(substr_vec : Vec<&str>) -> Option<IpAddr>{
    let mut iter = substr_vec.iter();
    let b1 = match String::from(*iter.next().unwrap()).parse::<u8>() {
        Ok(val) => val,
        _ => return None,
    };
    let b2 = match String::from(*iter.next().unwrap()).parse::<u8>() {
        Ok(val) => val,
        _ => return None,
    };    
    let b3 = match String::from(*iter.next().unwrap()).parse::<u8>() {
        Ok(val) => val,
        _ => return None,
    };    
    let b4 = match String::from(*iter.next().unwrap()).parse::<u8>() {
        Ok(val) => val,
        _ => return None,
    };
    return Some(IpAddr::V4(Ipv4Addr::new(b1, b2, b3, b4)));
}

fn hex_string_to_int(hex_string: &str) -> Option<u16> {
    // Attempt to parse the hexadecimal string into a u64 integer
    match u16::from_str_radix(hex_string, 16) {
        // If parsing succeeds, return the parsed integer wrapped in Some
        Ok(parsed_int) => Some(parsed_int),
        // If parsing fails (invalid input), return None
        Err(_) => None,
    }
}

/// Expects a vector of legnth 6
fn try_parse_to_ipv6(substr_vec : Vec<&str>) -> Option<IpAddr>{
    let mut iter = substr_vec.iter();
    let b1 = match hex_string_to_int(*iter.next().unwrap()) {
        Some(val) => val,
        _ => return None,
    };
    let b2 = match hex_string_to_int(*iter.next().unwrap()) {
        Some(val) => val,
        _ => return None,
    };    
    let b3 = match hex_string_to_int(*iter.next().unwrap()) {
        Some(val) => val,
        _ => return None,
    };    
    let b4 = match hex_string_to_int(*iter.next().unwrap()) {
        Some(val) => val,
        _ => return None,
    };
    let b5 = match hex_string_to_int(*iter.next().unwrap()) {
        Some(val) => val,
        _ => return None,
    };
    let b6 = match hex_string_to_int(*iter.next().unwrap()) {
        Some(val) => val,
        _ => return None,
    };
    let b7 = match hex_string_to_int(*iter.next().unwrap()) {
        Some(val) => val,
        _ => return None,
    };
    let b8 = match hex_string_to_int(*iter.next().unwrap()) {
        Some(val) => val,
        _ => return None,
    };
    return Some(IpAddr::V6(Ipv6Addr::new(b1, b2, b3, b4, b5, b6, b7, b8)));
}

