
pub struct Flag {
    pub name : &'static str,
    pub flag : &'static str,
    pub explanation: &'static str,
    pub arg_type: FlagArgumentType,
}

pub enum FlagArgumentType {
    None,
    UnsignedInt,
    String,
    OptionalString,
    OptionalUInt,
}

#[derive(PartialEq)]
pub enum FlagArg {
    UnsignedInt(u32),
    String(String),
    None,
}