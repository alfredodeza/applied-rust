//! This is a library that provides utilities for command-line tools.
//! So far it only provides a function to read a line from stdin.
//! # Examples:
//! ```
//! use cli_utils::read_stdin;
//! let input = read_stdin();
//! ```
//! # Panics:
//! The `read_stdin` function will panic if it fails to read a line with a message "Failed to read input line".

use std::io::{BufRead, BufReader};

pub mod config;
pub mod colors;


/// This function reads a line from stdin and returns it as a String.
/// It will panic if it fails to read a line with a message "Failed to read input line".
/// # Examples:
/// ```
/// use cli_utils::read_stdin;
/// let input = read_stdin();
/// ```
pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    _read_stdin(&mut reader)
}

fn _read_stdin<R: BufRead>(reader: &mut R) -> String {
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect("Failed to read input line");
    line.trim().to_string()
}

#[cfg(test)]
mod tests {
   use super::_read_stdin;
   use std::io::Cursor;

   #[test]
   fn test_read_input() {
       let input = "Hello, world!\n";
       let expected_output = "Hello, world!";
       let mut reader = Cursor::new(input);
       let output = _read_stdin(&mut reader);
       assert_eq!(output, expected_output);
   }

   #[test]
   fn test_read_input_empty() {
       let input = "";
       let expected_output = "";
       let mut reader = Cursor::new(input);
       let output = _read_stdin(&mut reader);
       assert_eq!(output, expected_output);
   }

}
