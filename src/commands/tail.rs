use crate::structs::command::Command;

pub const TAIL_CMD : &'static Command = &Command{
    cmd_arg: "tail",
    explanation: "Print a file's last ten lines",
    options: &[],
    func: tail_cmd,
};

pub fn tail_cmd() {

}