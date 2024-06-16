
pub struct Flag {
    pub name : &'static str,
    pub flag : &'static str,
    pub explanation: &'static str,
    pub arg_type: FlagArgumentType,
}

pub enum FlagArgumentType {
    None,
    Int,
    String,
    OptionalString,
    OptionalInt,
}

pub enum FlagArg {
    Int(i32),
    String(String),
    None,
}