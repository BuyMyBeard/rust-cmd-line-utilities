use super::flag::Flag;

pub struct Command {
    pub name: &'static str,
    pub command: &'static str,
    pub explanation: &'static str,
    pub options: &'static [&'static Flag],
    pub func: fn(),
}