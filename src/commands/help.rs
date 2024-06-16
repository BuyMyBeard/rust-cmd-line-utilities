use crate::{commands::command_list::COMMANDS, structs::command::Command};

pub const HELP_CMD : &'static Command = &Command{
    name: "Help",
    explanation: "Display this menu",
    command: "help",
    options: &[],
    func: help_cmd,
};

fn help_cmd() {

}

pub fn print_help_menu() {
    let mut t = term::stdout().unwrap();
    const TAB_SPACE: u8 = 10;
    const HELP_MENU_HEADER: &str = "For more information on a command, type 'help {command-name}'";
    let _ = writeln!(t, "{HELP_MENU_HEADER}");
    for cmd in COMMANDS {
        let _ = t.attr(term::Attr::Bold);
        let _ = write!(t, "{}", cmd.command);
        // For now, every command will use 1 byte characters, so len() give same result
        // let length = cmd.cmd_arg.chars().collect::<Vec<char>>().len();
        let length = cmd.command.len();
        let remaining = " ".repeat(TAB_SPACE as usize - length);
        let _ = t.reset();
        let _ = writeln!(t, "{}", remaining + cmd.explanation);
    }
}