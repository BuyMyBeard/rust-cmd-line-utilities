pub struct Command {
    pub cmd_arg: &'static str,
    pub explanation: &'static str,
    pub options: &'static [&'static str],
    pub func: fn(),
}