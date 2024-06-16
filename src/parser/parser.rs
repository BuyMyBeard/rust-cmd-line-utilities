use std::{any::type_name, process::exit};

use crate::{commands::{command_list::COMMANDS, help::print_help_menu}, structs::{command::Command, flag::{Flag, FlagArg, FlagArgumentType}}, utils::errors::{terminate_incorrect_format_error, terminate_invalid_flag_error, terminate_missing_argument_error, terminate_unknown_cmd_error}};

pub fn parse_command() -> &'static Command {
    let cmd_name = match std::env::args().nth(1) {
        Some(cmd) => cmd,
        None => {
            print_help_menu();
            exit(0);
        }
    };
    return match COMMANDS.iter().find(|command| command.command == cmd_name) {
        Some(cmd) => cmd,
        None => terminate_unknown_cmd_error(&cmd_name),
    };
}

/// Panics if current_arg is None or empty string.
fn parse_flag(flag_arg: String, next_arg : Option<String>, command : Command) -> (&'static Flag, FlagArg) {
    
    let flag = match command.options.iter().find(|x| x.flag == flag_arg) {
        Some(flag) => flag,
        None => terminate_invalid_flag_error(&flag_arg, command.name),
    };
    match flag.arg_type {
        FlagArgumentType::None => return (flag, FlagArg::None),
        FlagArgumentType::Int => {
            let arg = match next_arg {
                None => terminate_missing_argument_error(
                    command.name, 
                    flag.name, 
                    type_name::<i32>()
                ),
                Some(value) => value,
            };
            return match arg.parse::<i32>() {
                Ok(n) => (flag, FlagArg::Int(n)),
                Err(e) => terminate_incorrect_format_error(command.name, &arg, type_name::<i32>()),
            };
        },
        FlagArgumentType::OptionalInt => {
            let arg = match next_arg {
                None => return (flag, FlagArg::None),
                Some(value) => value,
            };
            return match arg.parse::<i32>() {
                Ok(n) => (flag, FlagArg::Int(n)),
                Err(e) => terminate_incorrect_format_error(command.name, &arg, type_name::<i32>()),
            };
        },
        FlagArgumentType::String => {
            return match next_arg {
                None => terminate_missing_argument_error(
                    command.name, 
                    flag.name, 
                    type_name::<String>()
                ),
                Some(value) => (flag, FlagArg::String(value)),
            };
        },
        FlagArgumentType::OptionalString => {
            return match next_arg {
                None => (flag, FlagArg::None),
                Some(value) => (flag, FlagArg::String(value)),
            };
        },
    }
}

trait IsFlag {
    fn is_flag(&self) -> Option<String>;
}

/// Returns Some() without the prefix or None if not a flag
impl IsFlag for Option<String> {
    fn is_flag(&self) -> Option<String> {
        let string = match &self {
            Some(value) => value,
            None => return None,
        };
        let first_char = match string.chars().nth(0) {
            Some(value) => value,
            None => return None,
        };
        return match first_char {
            '-' => Some(String::from(&string[1..string.len()])),
            _ => None,
        };
    }
}