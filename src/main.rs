use parser::parser::{parse_args, parse_command};

extern crate term;

mod commands;
mod structs;
mod utils;
mod parser;

fn main() {
    let command = parse_command();
    let (options, arguments) = parse_args(command);
    (command.func)(&options, &arguments);
}