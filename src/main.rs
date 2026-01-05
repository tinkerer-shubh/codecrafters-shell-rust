#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, fs};
use std::path::PathBuf;
use std::process::Command;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

fn find_executable_in_path(name: &str) -> Option<PathBuf> {
    let path_os = env::var_os("PATH")?;
    for dir in env::split_paths(&path_os) {
        let candidate = dir.join(name);

        let metadata = match fs::metadata(&candidate) {
            Ok(m) => m,
            Err(_) => continue,
        };

        if !metadata.is_file() {
            continue;
        }

        #[cfg(unix)]
        {
            if metadata.permissions().mode() & 0o111 == 0 {
                continue; // exists but not executable
            }
        }

        return Some(candidate);
    }
    None
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command_line = String::new();
        if io::stdin().read_line(&mut command_line).unwrap() == 0 {
            break;
        }

        let cmd = command_line.trim();
        if cmd.is_empty() {
            continue;
        }

        let parts: Vec<&str> = cmd.split_whitespace().collect();
        let program = parts[0];

        // exit
        if program == "exit"
        break;
    }
    
    // echo
    if program == "echo" {
        if parts.len() > 1{
            println!("{}", parts[1..].join(" "));
        } else {
            println!();
        }
        continue;
    }

    // type
    if program == "type" {
        if parts.len() < 2 {
            println!("type: not found");
            continue;
        }

        let target = parts[1];

        // buitins
        if matches!(target, "echo" | "exit" | "type") {
            println!("{} is a shell builtin", target);
            continue;
        }

        // path execs
        if let Some(path) = find_executable_in_path(target) {
            println!("{ is {}", target, path.display());
        } else {
            println!("{}: not found", target);
        }

        continue;
    }

    // external programs
    if let Some(path) = find_executable_in_path(program) {
        let mut child_cmd = Command::new(&path);

        child_cmd.arg0(program);

        if parts.len() > 1 {
            child_cmd.args(&parts[1..]);
        }

        match child_cmd.status() {
            Ok(_status) => {}
            Err(_e) => {
                // if exec fails for some reason
                println!("{}: command not found", program);
            }
        } else {
            println!("{: command not found", cmd);
        }
    }
}