//! This is a library that provides utilities for stdin
//! # Example
//! ```
//! let input = read_stdin();
//! use cli_utils::read_stdin;
//! ```
use std::io::{BufReader, BufRead};
pub mod colors;
pub mod config;

/// This function reads a line from stdin and returns it as a String
/// 
pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();
    reader.read_line(&mut line).expect("failed to read input line");
    line.trim().to_string()
}
