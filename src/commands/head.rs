use crate::structs::{command::Command, flag::{Flag, FlagArg}};

pub const HEAD_CMD : &'static Command = &Command{
    name: "Head",
    command: "head",
    explanation: "Print a file's first ten lines",
    options: &[],
    func: head_cmd,
};

pub fn head_cmd(options : &Vec::<(&'static Flag, FlagArg)>, arguments: &Vec::<String>) {

}