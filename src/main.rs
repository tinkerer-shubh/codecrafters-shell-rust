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

        println!("{}: command not found", cmd);
    }
}