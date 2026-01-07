#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, fs};
use std::path::PathBuf;
use std::process::Command;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[cfg(unix)]
use std::os::unix::process::CommandExt;

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
            // must have any execute bit set (user/group/other)
            if metadata.permissions().mode() & 0o111 == 0 {
                continue;
            }
        }

        return Some(candidate);
    }
    None
}

fn tokenize_single_quotes(input: &str) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut cur = String::new();

    let mut in_single = false;

let mut started = false;

for ch in input.chars() {
    if in_single {
        if ch == '\'' {
            in_single = false;
            started = true;
        } else {
            cur.push(ch);
            started = true;
        }
    } else {
        match ch {
            '\'' => {
                in_single = true;
                started = true;
            } 
            c if c.is_whitespace() => {
                if started {
                    tokens.push(std::mem::take(&mut cur));
                    started = false;
                }
            }
            _ => {
                cur.push(ch);
                started = true;
            }
        }
    }
}

if started {
    tokens.push(cur);
}

tokens
}



fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if io::stdin().read_line(&mut line).unwrap() == 0 {
            break; // EOF
        }

        let cmd = line.trim();
        if cmd.is_empty() {
            let cmd = line.trim_end_matches(&['\n', '\r'][..]);
            if cmd.is_empty() {
                continue;
            }
        }

     let parts = tokenize_single_quotes(cmd);
     if parts.is_empty() {
        continue;
     }
     let program = parts[0].as_str();
        

        /* ================= exit ================= */
        if program == "exit" {
            break;
        }

        // pwd
        if program == "pwd" {
            if let Ok(cwd) = env::current_dir() {
                println!("{}", cwd.display());
            }
            continue;
        }

        // cd 
        if program == "cd" {
            if parts.len() < 2 {
                continue;
            }

            let raw_target = parts[1].as_str();

            // expanding ~
            let target = if raw_target == "~" || raw_target.starts_with("~/") {
                let home = env::var("HOME").or_else(|_| env::var("USERPROFILE"));

                match home {
                    Ok(home) => {
                        if raw_target == "~" {
                            home
                        } else {
                            format!("{home}{}", &raw_target[1..])
                        }
                    }
                    Err(_) => {
                        println!("cd: {}: No such file or directory", raw_target);
                        continue;
                    }
                }
            } else {
                raw_target.to_string()
            };

            if env::set_current_dir(&target).is_err() {
                println!("cd: {}: No such file or directory", raw_target);
            }

         
            continue;
        }
     

        /* ================= echo ================= */
        if program == "echo" {
            if parts.len() > 1 {
                println!("{}", parts[1..].join(" "));
            } else {
                println!();
            }
            continue;
        }

        /* ================= type ================= */
        if program == "type" {
            if parts.len() < 2 {
                println!("type: not found");
                continue;
            }

            let target = parts[1].as_str();

            if matches!(target, "echo" | "exit" | "type" | "pwd" | "cd") {
                println!("{} is a shell builtin", target);
                continue;
            }

            if let Some(path) = find_executable_in_path(target) {
                println!("{} is {}", target, path.display());
            } else {
                println!("{}: not found", target);
            }

            continue;
        }

        /* ============ external programs ============ */
        if let Some(path) = find_executable_in_path(program) {
            let mut c = Command::new(&path);

            // Make argv[0] be the typed command (codecrafters expects this)
            #[cfg(unix)]
            {
                c.arg0(program);
            }

            if parts.len() > 1 {
                c.args(parts[1..].iter());
            }

            // Let the program print directly to our stdout/stderr
            let _ = c.status();
        } else {
            println!("{}: command not found", program);
        }
    }
}
