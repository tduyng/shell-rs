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

        if command.starts_with("exit") {
            exit_command(&command);
        }

        if !is_valid_command(command) {
            println!("{}: command not found", command)
        }
    }
}

fn is_valid_command(_command: &str) -> bool {
    false
}

fn exit_command(command: &str) {
    let exit_status = match command.split_whitespace().nth(1) {
        Some(status) => match status.parse::<i32>() {
            Ok(code) if code >= 0 && code <= 255 => code,
            _ => {
                eprintln!("Invalid exit status, using default value (0)");
                0
            }
        },
        None => 0,
    };

    std::process::exit(exit_status);
}