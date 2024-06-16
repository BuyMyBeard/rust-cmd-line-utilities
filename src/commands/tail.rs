use crate::structs::{command::Command, flag::{Flag, FlagArg}};

pub const TAIL_CMD : &'static Command = &Command{
    name: "Tail",
    command: "tail",
    explanation: "Print a file's last ten lines",
    options: &[],
    func: tail_cmd,
};

pub fn tail_cmd(options : &Vec::<(&'static Flag, FlagArg)>, arguments: &Vec::<String>) {
    todo!();
}