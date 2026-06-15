// Description: Implementation of shell commands
use std::path::Path;
use crate::env_vars::{get_vars};

pub fn cd_cmd(path: &str) -> std::io::Result<()> {
    let home = get_vars("HOME");
    let parts: Vec<&str> = path.split('~').collect();
    if parts.len() > 1 {
        let dest = parts[1];
        let new_path = format!("{}{}", home, dest);
        return std::env::set_current_dir(new_path)
    }
    match path {
        "~" => std::env::set_current_dir(home)?,
          _ => std::env::set_current_dir(path)?,
    }
    Ok(())
}

pub fn echo_cmd(string: &str) {
    println!("{string}");
}

pub fn pwd_cmd() -> std::io::Result<()> {
    let cwd = std::env::current_dir()?;
    println!("{}",cwd.display());
    Ok(())
}

pub fn type_cmd(s: &str) -> String {
    match s {
        "echo" | "exit" | "type" |
        "cd"   | "pwd" => format!("{s} is a shell builtin"),
        _ => {
            for path in super::get_paths(s) {
                if Path::new(&path).exists() && super::is_executable(Path::new(&path)) {
                    return format!("{s} is {}", path);
                }
            }
            format!("{s}: not found")
        }
    }
}
