use std::process::Command as ProcessCommand;

pub struct ExternalCommand {
    program: String,
    args: Vec<String>,
}

impl ExternalCommand {
    pub fn new(program: String, args: Vec<String>) -> Self {
        Self { program, args }
    }

    pub fn execute(&self) -> Result<(), String> {
        let output = ProcessCommand::new(&self.program)
            .args(&self.args)
            .output()
            .map_err(|e| format!("Failed to execute command: {}", e))?;

        if !output.stdout.is_empty() {
            print!("{}", String::from_utf8_lossy(&output.stdout));
        }
        if !output.stderr.is_empty() {
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        }

        Ok(())
    }
}
