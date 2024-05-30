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
        let new_path = if self.path == PathBuf::from("~") {
            env::var("HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("/"))
        } else if self.path.is_absolute() {
            self.path.clone()
        } else {
            env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("/"))
                .join(&self.path)
        };

        if new_path.exists() {
            env::set_current_dir(&new_path)
                .map_err(|_| format!("cd: {}: No such file or directory", new_path.display()))
        } else {
            Err(format!(
                "cd: {}: No such file or directory",
                new_path.display()
            ))
        }
    }
}
