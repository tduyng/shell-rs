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

        if let Err(err) = execute_command(command) {
            println!("{}", err);
        }
    }
}

fn execute_command(command: &str) -> Result<(), String> {
    let command_name = command.split_whitespace().next().unwrap_or_default();
    match command_name {
        "exit" => exit_command(command)?,
        "echo" => echo_command(command)?,
        "type" => type_command(command)?,
        _ => println!("{}: command not found", command),
    }

    Ok(())
}

fn exit_command(command: &str) -> Result<(), String> {
    let exit_status = command
        .split_whitespace()
        .nth(1)
        .unwrap_or_default()
        .parse::<i32>()
        .map_err(|_| "Invalid exit status")?;

    if !(0..=255).contains(&exit_status) {
        eprintln!("Invalid exit status, using default value (0)");
    }
    std::process::exit(exit_status);
}

fn echo_command(command: &str) -> Result<(), String> {
    let text = command.trim_start_matches("echo ");
    println!("{}", text);
    Ok(())
}

fn type_command(command: &str) -> Result<(), String> {
    let command_name = command.trim_start_matches("type ");
    let builtins = ["exit", "echo", "type"];

    if builtins.contains(&command_name) {
        println!("{} is a shell builtin", command_name);
    } else {
        println!("{} not found", command_name);
    }

    Ok(())
}
