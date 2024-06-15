use crate::structs::command::Command;
use super::ls::LS_CMD;
use super::head::HEAD_CMD;
use super::tail::TAIL_CMD;

pub const COMMANDS : &'static [&'static Command] = &[
    LS_CMD,
    HEAD_CMD,
    TAIL_CMD,
];