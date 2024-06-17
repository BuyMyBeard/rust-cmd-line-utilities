use std::{fs::File, io::{BufRead, BufReader}, path::Path, ptr::eq};

use crate::{structs::{command::Command, flag::{Flag, FlagArg, FlagArgumentType}}, utils::errors::{terminate_cannot_open_file, terminate_missing_argument_error}};

pub const HEAD_CMD : &'static Command = &Command{
    name: "Head",
    command: "head",
    explanation: "Print a file's first ten lines",
    options: &[
        LINES,
    ],
    func: head_cmd,
};


const LINES : &'static Flag = &Flag{
    name: "Lines",
    flag: "-n",
    explanation: "-n num: Prints the first 'num' lines instead of first 10 lines. num is mandatory to be specified in command otherwise it displays an error.",
    arg_type: FlagArgumentType::UnsignedInt,
};

pub fn head_cmd(options : &Vec::<(&'static Flag, FlagArg)>, arguments: &Vec::<String>) {
    let path = Path::new(match arguments.get(0) {
        None => terminate_missing_argument_error(HEAD_CMD.name),
        Some(arg) => arg,
    });

    let lines = match options.iter().find(|tup| {
        // FIXME: Somehow not the same memory address.
        println!("LINES: {:p}", LINES);
        println!("tup: {:p}", tup.0);
        return eq(tup.0, LINES);
    }) {
        Some(tup) =>  match tup.1 {
            FlagArg::UnsignedInt(val) => val,
            _ => panic!(),
        }
        None => 10,
    };
    
    let mut buf_reader = BufReader::new(match File::open(&path) {
        Err(e) => terminate_cannot_open_file(HEAD_CMD.name, path.display().to_string().as_str(), e),
        Ok(file) => file,
    });

    let mut t = term::stdout().unwrap();

    for _ in 0..lines {
        let mut line = String::new();
        match buf_reader.read_line(&mut line) {
            Ok(bytes) => if bytes == 0 {break}
            Err(e) => terminate_cannot_open_file(HEAD_CMD.name, path.display().to_string().as_str(), e),
        }
        _ = write!(t, "{}", line);
    }
}