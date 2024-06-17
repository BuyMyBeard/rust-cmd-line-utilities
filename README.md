# rust-cmd-line-utilities
Command line utility functions programmed in Rust to learn the language.

Includes an argument parser that supports options with arguments, and can parse arguments and options in any order.
Easily extendable command system allowing to add new commands and options easily.
Also supports extendable error handling for invalid arguments.

## Supported commands
This is still a work in progress.

### help
Display the help menu.

### ls (Not implemented yet)
Display the files and folders in the provided directory.

### head
Display the first 10 lines of a file.

-n {num}: Prints the first 'num' lines instead of first 10 lines. num is mandatory to be specified in command otherwise it displays an error.

### tail (Not implemented yet)
Display the last 10 lines of a file.
