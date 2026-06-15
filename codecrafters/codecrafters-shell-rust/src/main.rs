#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::{env, fs};
use std::path::Path;

mod cmds;
mod env_vars;
mod utils;

const PROMPT: &str = "$ ";

fn is_executable(path: &Path) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        let permissions = metadata.permissions();
        let mode = permissions.mode();
        mode & 0o111 != 0
    } else {
        false
    }
}

fn get_paths(exe: &str) -> Vec<String> {
    let key = "PATH";
    match env::var_os(key) {
        Some(paths) => env::split_paths(&paths)
            .map(|path| path.join(exe).to_string_lossy().into_owned())
            .collect(),
        None => vec![format!("{key} is not defined in the environment.")],
    }
}

fn exe_program(name: &str, args: &[&str]) {
    match std::process::Command::new(name).args(args).status() {
        Ok(status) => {
            if !status.success() {
                eprintln!("Command exited with non-zero status");
            }
        }
        Err(e) => eprintln!("Failed to execute command: {}", e),
    }
}

fn shell_keyword_lookup(input: &str) {

    let mut parts = input.split_whitespace();

    match parts.next() {
        Some("cd") => {
            if let Some(path) = parts.next() {
                if let Err(_e) = cmds::cd_cmd(path) {
                    // not using e error to pass but will put back
                    println!("cd: {path}: No such file or directory");
                }
            } else {
                println!("cd: not enough arguments");
            }
        }
        Some("pwd") => {
            if let Err(e) = cmds::pwd_cmd() {
                eprintln!("pwd error: {}", e);
            }
        }
        Some("echo") => {
            let args = utils::parse_args(input);
            let rest = args[1..].join(" ");
            cmds::echo_cmd(&rest);
        }
        Some("type") => {
            if let Some(cmd) = parts.next() {
                println!("{}", cmds::type_cmd(cmd));
            } else {
                println!("type: not enough arguments");
            }
        }
        Some("exit") => {
            match parts.next() {
                Some(code) => match code.parse::<i32>() {
                    Ok(0) => std::process::exit(0),
                    Ok(n) => println!("Exit code {n} not supported"),
                    Err(_) => println!("Invalid exit code"),
                },
                None => println!("No exit code provided"),
            }
        }
        Some(cmd) => {
            let mut found = false;
            let args = utils::parse_args(input);
            if !args.is_empty() {
                let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
                for path in get_paths(cmd) {
                    if Path::new(&path).exists() && is_executable(Path::new(&path)) {
                        exe_program(arg_refs[0], &arg_refs[1..]);
                        found = true;
                        break;
                    }
                }
            } else {
                let args: Vec<&str> = parts.collect();
                for path in get_paths(input) {
                    if Path::new(&path).exists() && is_executable(Path::new(&path)) {
                        exe_program(input, &args);
                        found = true;
                        break;
                    }
                }
            }
            if !found {
                println!("{input}: command not found");
            }
        }
        _ => println!("{input}: command not found"),
    }
}

fn user_input() {
    loop {
        print!("{PROMPT}");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        shell_keyword_lookup(input);
    }
}

fn main() {
    user_input();
}
