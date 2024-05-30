use crate::utils::find_in_path;

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
