use cd_command::CdCommand;
use echo_command::EchoCommand;
use exit_command::ExitCommand;
use external_command::ExternalCommand;
use pwd_command::PwdCommand;
use type_command::TypeCommand;

use crate::utils::find_in_path;

mod cd_command;
mod echo_command;
mod exit_command;
mod external_command;
mod pwd_command;
mod type_command;

pub enum Command {
    Exit(ExitCommand),
    Echo(EchoCommand),
    Type(TypeCommand),
    Pwd(PwdCommand),
    Cd(CdCommand),
    External(ExternalCommand),
}

impl Command {
    pub fn parse(command: &str) -> Result<Self, String> {
        let mut parts = command.split_whitespace();
        let command_name = parts.next().unwrap_or_default();
        let args: Vec<String> = parts.map(|s| s.to_string()).collect();

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
            "pwd" => Ok(Self::Pwd(PwdCommand::new())),
            "cd" => {
                if let Some(path) = args.first() {
                    Ok(Self::Cd(CdCommand::new(path.clone())))
                } else {
                    Err("cd: missing argument".to_string())
                }
            }
            _ => {
                if let Some(path) = find_in_path(command_name) {
                    Ok(Self::External(ExternalCommand::new(
                        path.to_string_lossy().to_string(),
                        args,
                    )))
                } else {
                    Err(format!("{}: command not found", command_name))
                }
            }
        }
    }

    pub fn execute(&self) -> Result<(), String> {
        match self {
            Command::Exit(cmd) => cmd.execute(),
            Command::Echo(cmd) => cmd.execute(),
            Command::Type(cmd) => cmd.execute(),
            Command::Pwd(cmd) => cmd.execute(),
            Command::Cd(cmd) => cmd.execute(),
            Command::External(cmd) => cmd.execute(),
        }
    }
}
