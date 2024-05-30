use std::env;

pub struct PwdCommand {}

impl PwdCommand {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute(&self) -> Result<(), String> {
        match env::current_dir() {
            Ok(path) => {
                println!("{}", path.display());
                Ok(())
            }
            Err(e) => Err(format!("pwd: {}", e)),
        }
    }
}
