#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        // If stdin is closed (EOF), exit the shell.
        if io::stdin().read_line(&mut command).unwrap() == 0 {
            break;
        }

        let cmd = command.trim();
        if cmd.is_empty() {
            continue;
        }

        // exit builtin
        if cmd == "exit" {
            break; // or `return;`
        }

        // echo builtin
        if cmd.starts_with("echo") {
            let parts: Vec<&str> = cmd.split_whitespace().collect();
            if parts.len() > 1 {
                println!("{}", parts[1..].join(" "));
            } else {
                println!();
            }
            continue;
        }

        println!("{}: command not found", cmd);
    }
}