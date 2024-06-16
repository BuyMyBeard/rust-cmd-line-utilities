use crate::structs::command::Command;

pub const TAIL_CMD : &'static Command = &Command{
    name: "Tail",
    command: "tail",
    explanation: "Print a file's last ten lines",
    options: &[],
    func: tail_cmd,
};

pub fn tail_cmd() {

}