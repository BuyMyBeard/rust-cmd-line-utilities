use crate::structs::command::Command;

use super::errors::terminate_invalid_flag_error;

pub enum ArgType {
    None,
    Argument(String),
    Flag(String),
}

trait OptionArgType {
    fn get_arg_type(&self, next : &Self, command : Command);
}

impl OptionArgType for Option<String> {
    /// Transform an Option<String> returned by iterator.peek() or iterator.next()
    fn get_arg_type(&self, next : &Self, command : Command) {
        let arg = &self.clone().unwrap();
        let first_char = arg.chars().nth(0).unwrap();
        
        if first_char == '-' {
            let flag_arg = &arg[1..arg.len()];
            let flag = match command.options.iter().find(|x| x.flag == flag_arg) {
                Some(flag) => flag,
                None => terminate_invalid_flag_error(flag_arg, command.name),
            };
        }  
    }
}
