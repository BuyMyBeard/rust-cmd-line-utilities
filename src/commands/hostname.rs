use gethostname::gethostname;

use crate::{structs::{command::Command, flag::{Flag, FlagArg}}, utils::errors::terminate_too_many_arguments};

pub const HOSTNAME_CMD : &'static Command = &Command{
    name: "Host Name",
    command: "hostname",
    explanation: "Print the current host name",
    options: &[],
    func: hostname_cmd,
};

fn hostname_cmd(_ : &Vec::<(&'static Flag, FlagArg)>, arguments: &Vec::<String>) {
    if arguments.len() > 0 {
        terminate_too_many_arguments(HOSTNAME_CMD.name, arguments[0].as_str())
    }
    let host = gethostname().into_string().unwrap_or_else(|os_string| {
        os_string.to_string_lossy().into_owned()
    });
    println!("{}", host.as_str())
}