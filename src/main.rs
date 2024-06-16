use commands::help::print_help_menu;
use parser::parser::parse_command;

extern crate term;

mod commands;
mod structs;
mod utils;
mod parser;

fn main() {
    print!("{}", "I like apples");
    let command = parse_command();
    print!("{}", command.name);
}