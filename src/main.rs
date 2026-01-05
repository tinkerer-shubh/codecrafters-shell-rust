#[allow(unused_imports)]
use std::io::{self, Write};

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

        // tokenize once
        let parts: Vec<&str> = cmd.split_whitespace().collect();

        // exit builtin
        if parts[0] == "exit" {
            break;
        }

        // echo builtin
        if parts[0] == "echo" {
            if parts.len() > 1 {
                println!("{}", parts[1..].join(" "));
            } else {
                println!();
            }
            continue;
        }

        // type builtin
        if parts[0] == "type" {
            if parts.len() < 2 {
                println!("type: not found");
                continue;
            }

            let target = parts[1];
            match target {
                "echo" | "exit" | "type" => {
               _ => println!("{}: not found", target);
                }
            }
            continue;
        }

        // fallback
        println!("{}: command not found", cmd);
    }
}