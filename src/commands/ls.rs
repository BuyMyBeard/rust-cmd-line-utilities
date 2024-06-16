use crate::structs::{command::Command, flag::{Flag, FlagArg}};

pub const LS_CMD : &'static Command = &Command{
    name: "List",
    command: "ls",
    explanation: "Lists directory contents of files and directories",
    options: &[],
    func: ls_cmd,
};

pub fn ls_cmd(options : &Vec::<(&'static Flag, FlagArg)>, arguments: &Vec::<String>) {
    todo!();
}