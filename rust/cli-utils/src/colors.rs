//! ANSI color utilities for terminal output.
//!
//! # Examples
//! ```
//! use cli_utils::colors::{red,green,yellow,blue,magenta,cyan,white};
//! let red_text = red("This text will be red");
//! let green_text = green("This text will be green");
//! ```
/// This returns a string with ANSI escape codes to color text red.
/// # Examples
/// ```
/// use cli_utils::colors::red;
/// let colored_text = red("This text will be red");
/// assert_eq!(colored_text, "\x1b[31mThis text will be red\x1b[0m");
/// ```
pub fn red(s: &str) -> String {
    format!("\x1b[31m{s}\x1b[0m")
}

pub fn green(s: &str) -> String {
    format!("\x1b[32m{s}\x1b[0m")
}

pub fn yellow(s: &str) -> String {
    format!("\x1b[33m{s}\x1b[0m")
}

pub fn blue(s: &str) -> String {
    format!("\x1b[34m{s}\x1b[0m")
}

pub fn magenta(s: &str) -> String {
    format!("\x1b[35m{s}\x1b[0m")
}

pub fn cyan(s: &str) -> String {
    format!("\x1b[36m{s}\x1b[0m")
}

pub fn white(s: &str) -> String {
    format!("\x1b[37m{s}\x1b[0m")
}

pub fn bright_red(s: &str) -> String {
    format!("\x1b[91m{s}\x1b[0m")
}

pub fn bright_green(s: &str) -> String {
    format!("\x1b[92m{s}\x1b[0m")
}

pub fn bright_yellow(s: &str) -> String {
    format!("\x1b[93m{s}\x1b[0m")
}

pub fn bright_blue(s: &str) -> String {
    format!("\x1b[94m{s}\x1b[0m")
}

pub fn bright_magenta(s: &str) -> String {
    format!("\x1b[95m{s}\x1b[0m")
}

pub fn bright_cyan(s: &str) -> String {
    format!("\x1b[96m{s}\x1b[0m")
}

pub fn bright_white(s: &str) -> String {
    format!("\x1b[97m{s}\x1b[0m")
}
