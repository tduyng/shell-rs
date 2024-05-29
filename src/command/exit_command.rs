pub struct ExitCommand {
    status: i32,
}

impl ExitCommand {
    pub fn new(status: i32) -> Self {
        Self { status }
    }

    pub fn execute(&self) -> Result<(), String> {
        std::process::exit(self.status);
    }
}
