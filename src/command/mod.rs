use echo_command::EchoCommand;
use exit_command::ExitCommand;
use type_command::TypeCommand;

mod echo_command;
mod exit_command;
mod type_command;

pub enum Command {
    Exit(ExitCommand),
    Echo(EchoCommand),
    Type(TypeCommand),
}

impl Command {
    pub fn parse(command: &str) -> Result<Self, String> {
        let command_name = command.split_whitespace().next().unwrap_or_default();
        match command_name {
            "exit" => {
                let status = command
                    .split_whitespace()
                    .nth(1)
                    .unwrap_or_default()
                    .parse::<i32>()
                    .map_err(|_| "Invalid exit status")?;

                if !(0..=255).contains(&status) {
                    eprintln!("Invalid exit status, using default value (0)");
                }

                Ok(Self::Exit(ExitCommand::new(status)))
            }
            "echo" => {
                let text = command.trim_start_matches("echo ");
                Ok(Self::Echo(EchoCommand::new(text.to_string())))
            }
            "type" => {
                let cmd_name = command.trim_start_matches("type ");
                Ok(Self::Type(TypeCommand::new(cmd_name.to_string())))
            }
            _ => Err(format!("{}: command not found", command)),
        }
    }

    pub fn execute(&self) -> Result<(), String> {
        match self {
            Command::Exit(cmd) => cmd.execute(),
            Command::Echo(cmd) => cmd.execute(),
            Command::Type(cmd) => cmd.execute(),
        }
    }
}
