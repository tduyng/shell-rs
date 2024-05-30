use std::{env, fs, os::unix::fs::PermissionsExt, path::PathBuf};

pub fn find_in_path(command_name: &str) -> Option<PathBuf> {
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

pub fn is_executable(path: &PathBuf) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}
