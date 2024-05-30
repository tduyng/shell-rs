use std::{env, path::PathBuf};

pub struct CdCommand {
    path: PathBuf,
}

impl CdCommand {
    pub fn new(path: String) -> Self {
        Self {
            path: PathBuf::from(path),
        }
    }

    pub fn execute(&self) -> Result<(), String> {
        if self.path.is_absolute() {
            env::set_current_dir(&self.path)
                .map_err(|_| format!("cd: {}: No such file or directory", self.path.display()))
        } else {
            Err("cd: only absolute paths are supported in this stage".to_string())
        }
    }
}
