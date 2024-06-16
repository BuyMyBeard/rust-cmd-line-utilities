use std::{io::Stdout, process::exit};

use term::Terminal;

pub fn terminate_invalid_flag_error(flag: &str, cmd_name: &str) -> ! {
    let mut t = get_error_term();
    if writeln!(t, "{} : No existing flag of type '{}'.", cmd_name, flag).is_err() {
        println!("{} : No existing flag of type '{}'.", cmd_name, flag);
    }
    cleanup_and_exit(t);
}

pub fn terminate_unknown_cmd_error(cmd_name: &str) -> ! {
    let mut t = get_error_term();
    if writeln!(t, "{} : The term '{}' is not a recognized command.", cmd_name, cmd_name).is_err() {
        println!("{} : The term '{}' is not a recognized command.", cmd_name, cmd_name);
    }
    cleanup_and_exit(t);
}

pub fn terminate_incorrect_format_error(cmd_name: &str, arg: &str, expected_type: &str) -> ! {
    let mut t = get_error_term();
    if writeln!(t, "{} : Cannot convert value '{}' to type '{}'.", cmd_name, arg, expected_type).is_err() {
        println!("{} : Cannot convert value '{}' to type '{}'.", cmd_name, arg, expected_type);
    }
    cleanup_and_exit(t);
}

pub fn terminate_missing_argument_error(cmd_name: &str, flag_name: &str, expected_type: &str) -> ! {
    let mut t = get_error_term();
    if writeln!(t, "{} : Missing an argument for parameter '{}'. Specify a parameter of type '{}' and try again.", cmd_name, flag_name, expected_type).is_err() {
        println!("{} : Missing an argument for parameter '{}'. Specify a parameter of type '{}' and try again.", cmd_name, flag_name, expected_type);
    }
    cleanup_and_exit(t);
}

pub fn terminate_custom_error(message: &str) -> ! {
    let mut t = get_error_term();
    if writeln!(t, "{}", message).is_err() {
        println!("{}", message);
    }
    cleanup_and_exit(t);
}

fn get_error_term() -> Box<dyn Terminal<Output = Stdout> + Send> {
    let mut t = term::stdout().unwrap();
    let _ = t.fg(term::color::BRIGHT_RED);
    
    return t;
}

fn cleanup_and_exit(mut t : Box<dyn Terminal<Output = Stdout> + Send>) -> ! {
    let _ = t.reset();
    let _ = t.flush();
    exit(1);
}
