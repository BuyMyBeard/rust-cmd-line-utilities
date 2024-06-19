use std::net::IpAddr;


pub struct Flag {
    pub name : &'static str,
    pub flag : &'static str,
    pub explanation: &'static str,
    pub arg_type: FlagArgumentType,
}

#[derive(PartialEq)]
pub enum FlagArgumentType {
    None,
    UnsignedInt,
    IpAddress,
    String,
    OptionalString,
    OptionalUInt,
}

#[derive(PartialEq)]
pub enum FlagArg {
    UnsignedInt(u32),
    String(String),
    IpAddress(IpAddr),
    None,
}