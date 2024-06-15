use crate::structs::command::Command;

pub const HEAD_CMD : &'static Command = &Command{
    cmd_arg: "head",
    explanation: "Print a file's first ten lines",
    options: &[],
    func: head_cmd,
};

pub fn head_cmd() {

}