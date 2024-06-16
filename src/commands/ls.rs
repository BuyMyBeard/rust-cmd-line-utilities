use crate::structs::command::Command;

pub const LS_CMD : &'static Command = &Command{
    name: "List",
    command: "ls",
    explanation: "Lists directory contents of files and directories",
    options: &[],
    func: ls_cmd,
};

pub fn ls_cmd() {

}