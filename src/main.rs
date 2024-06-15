extern crate term;

fn main() {

    let mut args = std::env::args();
    args.next();

    let cmd_arg  = args.next();
    if cmd_arg == None {
        print_help_menu();
        return;
    }  
}

fn print_help_menu() {
    let mut t = term::stdout().unwrap();
    const TAB_SPACE: u8 = 10;
    const HELP_MENU_HEADER: &str = "For more information on a command, type 'help {command-name}'";
    let _ = writeln!(t, "{HELP_MENU_HEADER}");
    for cmd in COMMANDS {
        let _ = t.attr(term::Attr::Bold);
        let _ = write!(t, "{}", cmd.cmd_arg);
        let length = cmd.cmd_arg.chars().collect::<Vec<char>>().len();
        let remaining = " ".repeat(TAB_SPACE as usize - length);
        let _ = t.reset();
        let _ = writeln!(t, "{}", remaining + cmd.explanation);
    }
}

struct Command {
    cmd_arg : &'static str,
    explanation: &'static str,
    options: &'static [&'static str],
}

const COMMANDS : &'static [&'static Command] = &[
    &Command{
        cmd_arg: "ls",
        explanation: "Lists directory contents of files and directories",
        options: &[],
    },
    &Command{
        cmd_arg: "head",
        explanation: "Print a file's first ten lines",
        options: &[],
    },
    &Command{
        cmd_arg: "tail",
        explanation: "Print a file's last ten lines",
        options: &[],
    }
];