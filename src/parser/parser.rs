use std::{any::type_name, net::IpAddr};

use crate::{commands::{command_list::COMMANDS, help::print_help_menu}, structs::{command::Command, flag::{Flag, FlagArg, FlagArgumentType}}, utils::{errors::{terminate_incorrect_format_error, terminate_invalid_flag_error, terminate_missing_flag_argument_error, terminate_unknown_cmd_error}, utils::{is_flag, try_parse_string_to_ip}}};

pub fn parse_command() -> &'static Command {
    let cmd_name = String::from(match std::env::args().nth(1) {
        Some(cmd) => cmd,
        None => {
            print_help_menu();
        }
    }.trim());
    
    return match COMMANDS.iter().find(|command| command.command == cmd_name) {
        Some(cmd) => cmd,
        None => terminate_unknown_cmd_error(&cmd_name, &cmd_name),
    };
}

pub fn parse_args(command : &Command) -> (Vec::<(&'static Flag, FlagArg)>, Vec::<String>){
    let mut args_iterator = std::env::args().peekable();
    let mut options = Vec::<(&'static Flag, FlagArg)>::new();
    let mut arguments = Vec::<String>::new();
    
    args_iterator.nth(1);
    
    while let Some(arg) = args_iterator.next() {
        if is_flag(&arg) {
            let next_arg = args_iterator.peek().cloned();
            let option = parse_flag(String::from(arg.trim()), next_arg, command);
            if option.1 != FlagArg::None {
                args_iterator.next();
            }
            options.push(option);
            continue;
        }
        arguments.push(String::from(arg.trim()));
    }
    return (options, arguments);
}

fn parse_flag(flag_arg: String, next_arg : Option<String>, command : &Command) -> (&'static Flag, FlagArg) { 
    let flag = match command.options.iter().find(|x| x.flag == flag_arg) {
        Some(flag) => flag,
        None => terminate_invalid_flag_error(&flag_arg, command.name),
    };
    match flag.arg_type {
        FlagArgumentType::None => return (flag, FlagArg::None),
        FlagArgumentType::UnsignedInt => {
            let arg = String::from(match next_arg {
                None => terminate_missing_flag_argument_error(
                    command.name, 
                    flag.name, 
                    type_name::<u32>()
                ),
                Some(value) => value,
            }.trim());
            return match arg.parse::<u32>() {
                Ok(n) => (flag, FlagArg::UnsignedInt(n)),
                Err(_) => terminate_incorrect_format_error(command.name, &arg, type_name::<u32>()),
            };
        },
        FlagArgumentType::OptionalUInt => {
            let arg = String::from(match next_arg {
                None => return (flag, FlagArg::None),
                Some(value) => value,
            }.trim());
            return match arg.parse::<u32>() {
                Ok(n) => (flag, FlagArg::UnsignedInt(n)),
                Err(_) => terminate_incorrect_format_error(command.name, &arg, type_name::<u32>()),
            };
        },
        FlagArgumentType::String => {
            return match next_arg {
                None => terminate_missing_flag_argument_error(
                    command.name, 
                    flag.name, 
                    type_name::<String>()
                ),
                Some(value) => (flag, FlagArg::String(String::from(value.trim()))),
            };
        },
        FlagArgumentType::OptionalString => {
            return match next_arg {
                None => (flag, FlagArg::None),
                Some(value) => (flag, FlagArg::String(String::from(value.trim()))),
            };
        },
        FlagArgumentType::IpAddress => {
            let arg = String::from(match next_arg {
                None => terminate_missing_flag_argument_error(
                    command.name,
                    flag.name, 
                    type_name::<IpAddr>()
                ),
                Some(value) => value,
            }.trim());
            match try_parse_string_to_ip(&arg) {
                Some(ip) => return (flag, FlagArg::IpAddress(ip)),
                None => terminate_incorrect_format_error(command.name, &arg, type_name::<IpAddr>())
            };
        }
    }
}