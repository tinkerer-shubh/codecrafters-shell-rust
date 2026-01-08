#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, fs};
use std::path::PathBuf;
use std::process::{Command, Stdio};


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

fn tokenize(input: &str) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut cur = String::new();

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Mode {
        Normal,
        Single,
        Double,
    }

    let mut mode = Mode::Normal;
    let mut started = false;
    
  
        let mut it = input.chars();
        while let Some(ch) = it.next() { 
        match mode {
            Mode::Single => {
                if ch == '\'' {
                    mode = Mode::Normal;
                    started = true;
                } else {
                    cur.push(ch);
                    started = true;
                }
            }
            Mode::Double => {
                if ch == '"' {
                    mode = Mode::Normal;
                    started = true;
                } else if ch == '\\' {
                    if let Some(next) = it.next() {
                        match next {
                            '"' | '\\' => {
                                // \" -> "
                                // \\ -> \ 
                                cur.push(next);
                            }
                            _ => {
                                cur.push('\\');
                                cur.push(next);
                            }
                        }
                    } else {
                        cur.push('\\');
                    } 
                    started = true;
                } else {
                    cur.push(ch);
                    started = true;
                }
            }

            Mode::Normal => {
                if ch == '\\' {
                    if let Some(next) = it.next() {
                        cur.push(next);
                    } else {
                        cur.push('\\');
                    }
                    started = true;
                } else if ch == '\'' { 
                    mode = Mode::Single;
                    started = true;
                } else if ch == '"' {
                    mode = Mode::Double;
                    started = true;
                } else if ch == '>' {
                    // redirection operator
                    if started {
                        if cur == "1" {
                            tokens.push("1>".to_string());
                            cur.clear();
                            started = false;
                            continue;
                        } else {
                            tokens.push(std::mem::take(&mut cur));
                            started = false;
                        }
                    }
                    tokens.push(">".to_string());
                } else if ch.is_whitespace() {
                    if started {
                        tokens.push(std::mem::take(&mut cur));
                        started = false;
                    }
                } else {
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

fn split_stdout_redirection(tokens: &[String]) -> (Vec<String>, Option<String>) {
    let mut argv: Vec<String> = Vec::new();
    let mut redirect: Option<String> = None;

    let mut i = 0usize;
    while i < tokens.len() {
        let t = tokens[i].as_str();

        if t == ">" || t == "1>" {
            if i + 1 < tokens.len() {
                redirect = Some(tokens[i + 1].clone());
                i += 2;
                continue;
            } else {
                argv.push(tokens[i].clone());
                i += 1;
                continue;
            }
        }

        if t == "1" && i + 1 < tokens.len() && tokens[i + 1] == ">" {
            if i + 2 < tokens.len() {
                redirect = Some(tokens[i + 2].clone());
                i += 3;
                continue;
            } else {
                argv.push(tokens[i].clone());
                argv.push(tokens[i + 1].clone());
                i += 2;
                continue;
            }
        }

        

        argv.push(tokens[i].clone());
        i += 1;
    }

    (argv, redirect)
}

fn write_line(out_file: Option<&mut fs::File>, line: &str) {
    if let Some(f) = out_file {
        let _ = writeln!(f, "{}", line);
    } else {
        println!("{}", line);
    }
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if io::stdin().read_line(&mut line).unwrap() == 0 {
            break; // EOF
        }

       let cmd = line.trim_end_matches(&['\n', '\r'][..]);
       if cmd.trim().is_empty() {
        continue;
       }
       let parts = tokenize(cmd);
       if parts.is_empty() {
        continue;
     }

     let (argv, stdout_redirect) = split_stdout_redirection(&parts);
     if argv.is_empty() {
        continue;
     }

     let program = argv[0].as_str();

     let mut out_file: Option<fs::File> = match stdout_redirect.as_deref() {
        Some(path) => match fs::File::create(path) {
            Ok(f) => Some(f),
            Err(e) => {
                // keep error on terminal
                println!("{}: {}", path, e);
                None
            }
        },
        None => None,
    
    };

        

        /* ================= exit ================= */
        if program == "exit" {
            break;
        }

        // pwd
        if program == "pwd" {
            if let Ok(cwd) = env::current_dir() {
               write_line(out_file.as_mut(), &format!("{}", cwd.display()));
            }
            continue;
        }

        // cd 
        if program == "cd" {
            if argv.len() < 2 {
                continue;
            }

            let raw_target = argv[1].as_str();

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
            if argv.len() > 1 {
                write_line(out_file.as_mut(), &argv[1..].join(" "));
            } else {
                write_line(out_file.as_mut(), "");
            }
            continue;
        }

        /* ================= type ================= */
        if program == "type" {
            if argv.len() < 2 {
                write_line(out_file.as_mut(), "type: not found");
                continue;
            }

            let target = argv[1].as_str();

            if matches!(target, "echo" | "exit" | "type" | "pwd" | "cd") {
                write_line(out_file.as_mut(), &format!("{} is a shell builtin", target));
                continue;
            }

            if let Some(path) = find_executable_in_path(target) {
               write_line(out_file.as_mut(), &format!("{} is {}", target, path.display()));
            } else {
               write_line(out_file.as_mut(), &format!("{}: not found", target));
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

            if argv.len() > 1 {
                c.args(argv[1..].iter());
            }

            // redirect only stdout when requested (stderr stays on terminal)
            if let Some(file) = out_file.take() {
                c.stdout(Stdio::from(file));
            }

            // Let the program print directly to our stdout/stderr
            let _ = c.status();
        } else {
            println!("{}: command not found", program);
        }
    }
}

