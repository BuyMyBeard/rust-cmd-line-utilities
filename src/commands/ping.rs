use std::{any::type_name, net::IpAddr};

use winping::{Buffer, Pinger};

use crate::{structs::{command::Command, flag::{Flag, FlagArg}}, utils::{errors::{terminate_incorrect_format_error, terminate_missing_argument_error, terminate_too_many_arguments, terminate_ping_error}, utils::try_parse_string_to_ip}};

pub const PING_CMD : &'static Command = &Command{
    name: "Ping",
    explanation: "Ping the server at the provided ip address. Works for ipv4 and ipv6",
    command: "ping",
    options: &[],
    func: ping_cmd,
};

fn ping_cmd(_ : &Vec::<(&'static Flag, FlagArg)>, arguments: &Vec::<String>) {
    if arguments.len() > 1 {
        terminate_too_many_arguments(PING_CMD.name, arguments[1].as_str())
    } else if arguments.is_empty() {
        terminate_missing_argument_error(PING_CMD.name);
    }
    let addr = match try_parse_string_to_ip(arguments[0].as_str()) {
        Some(ip) => ip,
        None => terminate_incorrect_format_error(PING_CMD.name, arguments[0].as_str(), type_name::<IpAddr>())
    };

    let pinger = Pinger::new().unwrap();
    let mut buffer = Buffer::new();

    let mut t = term::stdout().unwrap();

    for _ in 0..4 {
        match pinger.send(addr, &mut buffer) {
            Ok(res_time) => _ = writeln!(t, "Response time was {} ms", res_time),
            Err(e) => {
                _ = t.flush();
                terminate_ping_error(PING_CMD.name, e);
            }
        }
    }
    _ = t.flush();
}