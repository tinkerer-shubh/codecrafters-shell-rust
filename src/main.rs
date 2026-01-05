#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, fs, path::PathBuff};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        if io::stdin().read_line(&mut command).unwrap() == 0 {
            break;
        }

        let cmd = command.trim();
        if cmd.is_empty() {
            continue;
        }

        let parts: Vec<&str> = cmd.split_whitespace().collect();

        // exit
        if parts[0] == "exit" {
            break;
        }

        // echo
        if parts[0] == "echo" {
            if parts.len() > 1 {
                println!("{}", parts[1..].join(" "));
            } else {
                println!();
            }
            continue;
        }

        // type
        if parts[0] == "type" {
            if parts.len() < 2 {
                println!("type: not found");
                continue;
            }

            let target = parts[1];

            // builtins 
            if matches!(target, "echo" | "exit" | "type") {
                println!("{ is a shell builtin", target);
                continue;
            }

            let mut found: Option<PathBuf> = None;

            if let Some(path_os) = env::var_os("PATH") {
                for dir in env::split_paths(&path_os) {
                    let candidate = dir.join(target);

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
                            continue;
                        }
                    }

                    found = Some(candidate);
                    break;
                }
            }

            if let Some(path) == found {
                println!("{ is {}", target, path.display());
            } else {
                println!("{}: not found", target);
            }

            continue;
        }

        // fallback
        println!("{}: command not found", cmd);
    }
}