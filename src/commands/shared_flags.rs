use crate::structs::flag::{Flag, FlagArgumentType};

pub const VERBOSE : &'static Flag = &Flag{
    name: "Verbose",
    flag: "-v",
    explanation: "-v : Preceeds the data from the specified file by the file name.",
    arg_type: FlagArgumentType::None,
};