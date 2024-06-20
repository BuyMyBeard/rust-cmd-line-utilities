# rust-cmd-line-utilities
Command line utility functions programmed in Rust to learn the language.

Includes an argument parser that supports options with arguments, and can parse arguments and options in any order.
Easily extendable command system allowing to add new commands and options easily.
Also supports extendable error handling for invalid arguments.

## Supported commands
This is still a work in progress.

### help [command]
Display the help menu.
If 'command' is provided, displays specific help for this command and the different flags for it.

### ls {path} (Not implemented yet)
Display the files and folders in the provided directory.

### head {path}
Display the first 10 lines of a file.

-n {num}: Prints the first 'num' lines instead of first 10 lines. num is mandatory to be specified in command otherwise it displays an error.
-c {num}: Prints the up to 'num' bytes.
-v : Preceeds the file content by the file name.

### tail (Not implemented yet)
Display the last 10 lines of a file.

-n {num}: Prints the last 'num' lines instead of last 10 lines. num is mandatory to be specified in command otherwise it displays an error.
-c {num}: Prints the up to 'num' bytes.
-v : Preceeds the file content by the file name.

### ping {ipAddr}
Ping the server at the provided ip address. Works for ipv4 and ipv6.

### hostname
Print the current host name.

### echo [...args]
Print a message as a standard output.
