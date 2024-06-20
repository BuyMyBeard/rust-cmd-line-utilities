use crate::structs::command::Command;
use super::echo::ECHO_CMD;
use super::help::HELP_CMD;
use super::hostname::HOSTNAME_CMD;
use super::ls::LS_CMD;
use super::head::HEAD_CMD;
use super::ping::PING_CMD;
use super::tail::TAIL_CMD;

pub const COMMANDS : &'static [&'static Command] = &[
    LS_CMD,
    HEAD_CMD,
    TAIL_CMD,
    HELP_CMD,
    PING_CMD,
    HOSTNAME_CMD,
    ECHO_CMD,
];