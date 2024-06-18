use std::process::exit;

use crate::{commands::command_list::COMMANDS, structs::{command::Command, flag::{Flag, FlagArg}}, utils::errors::{terminate_too_many_arguments, terminate_unknown_cmd_error}};

pub const HELP_CMD : &'static Command = &Command{
    name: "Help",
    explanation: "Display this menu",
    command: "help",
    options: &[],
    func: help_cmd,
};

fn help_cmd(_ : &Vec::<(&'static Flag, FlagArg)>, arguments: &Vec::<String>) {
    if let Some(invalid_argument) = arguments.iter().nth(1) {
        terminate_too_many_arguments(&HELP_CMD.name, invalid_argument);
    }
    match arguments.iter().nth(0) {
        Some(arg) => print_cmd_help(arg.as_str()),
        None => print_help_menu(),
    }
}

pub fn print_cmd_help(cmd_str: &str) {
    let command = match COMMANDS.iter().find(|cmd| cmd.command == cmd_str) {
        Some(command) => command,
        None => terminate_unknown_cmd_error(HELP_CMD.name, cmd_str),
    };
    let mut t = term::stdout().unwrap();
    _ = t.attr(term::Attr::Bold);
    _ = t.fg(term::color::BRIGHT_BLUE);
    _ = writeln!(t, "{}", command.name);
    _ = t.reset();

    _ = writeln!(t, "{}", command.explanation);

    if command.options.is_empty() {
        _ = t.flush();
        return;
    }
    _ = t.attr(term::Attr::Bold);
    _ = writeln!(t, "\nOptional flags:");
    _ = t.reset();
    for flag in command.options.iter() {
        _ = writeln!(t, "{}", flag.name);
        _ = writeln!(t, "{}\n", flag.explanation);
    }
    _ = t.flush();
}

pub fn print_help_menu() -> ! {
    let mut t = term::stdout().unwrap();
    const TAB_SPACE: u8 = 10;
    const HELP_MENU_HEADER: &str = "For more information on a command, type 'help {command-name}'";
    let _ = writeln!(t, "{HELP_MENU_HEADER}");
    for cmd in COMMANDS {
        _ = t.attr(term::Attr::Bold);
        _ = write!(t, "{}", cmd.command);
        // For now, every command will use 1 byte characters, so len() give same result
        // let length = cmd.cmd_arg.chars().collect::<Vec<char>>().len();
        let length = cmd.command.len();
        let remaining = " ".repeat(TAB_SPACE as usize - length);
        _ = t.reset();
        _ = writeln!(t, "{}", remaining + cmd.explanation);
    }
    _ = t.reset();
    _ = t.flush();
    exit(0);
}