use std::io::{self, Write};

use command::Command;

mod command;
mod utils;

fn main() -> Result<(), String> {
    loop {
        print_prompt();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let command = input.trim();
        if command.is_empty() {
            continue;
        }

        if let Err(e) = execute_command(command) {
            eprintln!("{}", e);
        }
    }
}

fn print_prompt() {
    print!("$ ");
    io::stdout().flush().unwrap();
}

fn execute_command(command: &str) -> Result<(), String> {
    let command = Command::parse(command)?;
    command.execute()
}
