use crate::structs::{command::Command, flag::{Flag, FlagArg}};

pub const LS_CMD : &'static Command = &Command{
    name: "List",
    command: "ls",
    explanation: "Lists directory contents of files and directories",
    options: &[],
    func: ls_cmd,
};

pub fn ls_cmd(_ : &Vec::<(&'static Flag, FlagArg)>, _: &Vec::<String>) {
    todo!();
}