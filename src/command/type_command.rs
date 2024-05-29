use std::{env, fs, os::unix::fs::PermissionsExt, path::PathBuf};

pub struct TypeCommand {
    command_name: String,
}

impl TypeCommand {
    pub fn new(command_name: String) -> Self {
        Self { command_name }
    }

    pub fn execute(&self) -> Result<(), String> {
        let builtins = ["exit", "echo", "type"];

        if builtins.contains(&self.command_name.as_str()) {
            println!("{} is a shell builtin", self.command_name);
        } else if let Some(path) = find_in_path(&self.command_name) {
            println!("{}", path.display());
        } else {
            println!("{} not found", self.command_name);
        }

        Ok(())
    }
}

fn find_in_path(command_name: &str) -> Option<PathBuf> {
    let path_var = env::var("PATH").unwrap_or_default();
    let paths = env::split_paths(&path_var);

    for dir in paths {
        let full_path = dir.join(command_name);
        if full_path.is_file() && is_executable(&full_path) {
            return Some(full_path);
        }
    }

    None
}

fn is_executable(path: &PathBuf) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}
