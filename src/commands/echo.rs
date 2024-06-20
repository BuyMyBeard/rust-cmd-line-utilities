use crate::structs::{command::Command, flag::{Flag, FlagArg}};

pub const ECHO_CMD : &'static Command = &Command{
    name: "Echo",
    command: "echo",
    explanation: "Print a message as a standard output",
    options: &[],
    func: echo_cmd,
};

fn echo_cmd(_ : &Vec::<(&'static Flag, FlagArg)>, arguments: &Vec::<String>) {
    if arguments.len() == 0 {
        println!("ECHO is on");
    }
    for arg in arguments {
        print!("{} ", arg);
    }
    println!();
}