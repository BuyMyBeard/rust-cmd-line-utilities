use std::{fs::File, io::{BufRead, BufReader}, path::Path};

use crate::{structs::{command::Command, flag::{Flag, FlagArg, FlagArgumentType}}, utils::{errors::{terminate_cannot_open_file, terminate_missing_argument_error}, utils::{flag_u32_arg_or_default, flag_u32_arg_or_none}}};

pub const HEAD_CMD : &'static Command = &Command{
    name: "Head",
    command: "head",
    explanation: "Print a file's first ten lines",
    options: &[
        LINES,
        BYTES,
    ],
    func: head_cmd,
};

const LINES : &'static Flag = &Flag{
    name: "Lines",
    flag: "-n",
    explanation: "-n num: Prints the first 'num' lines instead of first 10 lines. 'num' is mandatory to be specified in command otherwise it displays an error.",
    arg_type: FlagArgumentType::UnsignedInt,
};

const BYTES : &'static Flag = &Flag{
    name: "Bytes",
    flag: "-c",
    explanation: "-c num: Prints up to 'num' bytes. 'num' is mandatory to be specified in command otherwise it displays an error.",
    arg_type: FlagArgumentType::UnsignedInt,
};

pub fn head_cmd(options : &Vec::<(&'static Flag, FlagArg)>, arguments: &Vec::<String>) {
    let path = Path::new(match arguments.get(0) {
        None => terminate_missing_argument_error(HEAD_CMD.name),
        Some(arg) => arg,
    });

    let lines = flag_u32_arg_or_default(options, LINES, 10);
    let max_bytes = flag_u32_arg_or_none(options, BYTES);
    
    let mut buf_reader = BufReader::new(match File::open(&path) {
        Err(e) => terminate_cannot_open_file(HEAD_CMD.name, path.display().to_string().as_str(), e),
        Ok(file) => file,
    });

    let mut t = term::stdout().unwrap();

    const UNDISPLAYABLE: &'static str = "Undisplayable file name";
    let file_name = match path.file_name() {
        Some(name) => name.to_str().unwrap_or(UNDISPLAYABLE),
        None => UNDISPLAYABLE,
    };

    _ = writeln!(t, "==> {} <==", file_name);

    let mut total_bytes_read: u32 = 0;

    for _ in 0..lines {
        let mut line = String::new();
        match buf_reader.read_line(&mut line) {
            Ok(bytes_read) => {
                if bytes_read == 0 {break}
                total_bytes_read += bytes_read as u32;
            }
            Err(e) => terminate_cannot_open_file(HEAD_CMD.name, path.display().to_string().as_str(), e),
        }
        if max_bytes.is_some_and(|max| total_bytes_read > max) {
            let remaining = line.len() as u32 - (total_bytes_read - max_bytes.unwrap());
            _ = write!(t, "{}", &line[0..remaining as usize]);
            break;
        }
        _ = write!(t, "{}", line);
    }
}