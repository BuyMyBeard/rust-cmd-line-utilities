use std::{fs::File, io::{BufRead, BufReader, Seek, SeekFrom}, path::Path};

use crate::{structs::{command::Command, flag::{Flag, FlagArg, FlagArgumentType}}, utils::{errors::{terminate_cannot_open_file, terminate_missing_argument_error}, utils::{flag_u32_arg_or_default, flag_u32_arg_or_none, has_flag}}};

use super::shared_flags::VERBOSE;

pub const TAIL_CMD : &'static Command = &Command{
    name: "Tail",
    command: "tail",
    explanation: "Print a file's last ten lines",
    options: &[
        LINES,
        BYTES,
        VERBOSE,
    ],
    func: tail_cmd,
};

const LINES : &'static Flag = &Flag{
    name: "Lines",
    flag: "-n",
    explanation: "-n num : Prints the last 'num' lines instead of last 10 lines. 'num' is mandatory to be specified in command otherwise it displays an error.",
    arg_type: FlagArgumentType::UnsignedInt,
};

const BYTES : &'static Flag = &Flag{
    name: "Bytes",
    flag: "-c",
    explanation: "-c num : Prints up to 'num' bytes. 'num' is mandatory to be specified in command otherwise it displays an error.",
    arg_type: FlagArgumentType::UnsignedInt,
};

pub fn tail_cmd(options : &Vec::<(&'static Flag, FlagArg)>, arguments: &Vec::<String>) {
    todo!();
    // let path = Path::new(match arguments.get(0) {
    //     None => terminate_missing_argument_error(TAIL_CMD.name),
    //     Some(arg) => arg,
    // });

    // let lines = flag_u32_arg_or_default(options, LINES, 10);
    // let max_bytes = flag_u32_arg_or_none(options, BYTES);
    
    // let mut buf_reader = BufReader::new(match File::open(&path) {
    //     Err(e) => terminate_cannot_open_file(TAIL_CMD.name, path.display().to_string().as_str(), e),
    //     Ok(file) => file,
    // });

    // // _ = buf_reader.seek(SeekFrom::End(0));

    // let mut t = term::stdout().unwrap();

    // if has_flag(options, VERBOSE) {
    //     const UNDISPLAYABLE: &'static str = "Undisplayable file name";
    //     let file_name = match path.file_name() {
    //         Some(name) => name.to_str().unwrap_or(UNDISPLAYABLE),
    //         None => UNDISPLAYABLE,
    //     };
    //     _ = writeln!(t, "==> {} <==", file_name);
    // } 

    // let mut total_bytes_read: u32 = 0;

    // for _ in 0..lines {
    //     let mut line = String::new();
    //     match buf_reader.read_line(&mut line) {
    //         Ok(bytes_read) => {
    //             if bytes_read == 0 {break}
    //             total_bytes_read += bytes_read as u32;
    //         }
    //         Err(e) => terminate_cannot_open_file(TAIL_CMD.name, path.display().to_string().as_str(), e),
    //     }
    //     if max_bytes.is_some_and(|max| total_bytes_read > max) {
    //         let remaining = line.len() as u32 - (total_bytes_read - max_bytes.unwrap());
    //         _ = write!(t, "{}", &line[0..remaining as usize]);
    //         break;
    //     }
    //     _ = write!(t, "{}", line);
    // }
}