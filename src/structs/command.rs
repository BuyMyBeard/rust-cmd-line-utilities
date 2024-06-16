use super::flag::{Flag, FlagArg};

pub struct Command {
    pub name: &'static str,
    pub command: &'static str,
    pub explanation: &'static str,
    pub options: &'static [&'static Flag],
    pub func: fn(options : &Vec::<(&'static Flag, FlagArg)>, arguments: &Vec::<String>),
}