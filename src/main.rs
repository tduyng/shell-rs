#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let command = input.trim();
        if command.is_empty() {
            continue;
        }

        if !is_valid_command(command) {
            println!("{}: command not found", command)
        }
    }
}

fn is_valid_command(_command: &str) -> bool {
    false
}
