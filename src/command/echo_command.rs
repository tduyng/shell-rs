pub struct EchoCommand {
    text: String,
}

impl EchoCommand {
    pub fn new(text: String) -> Self {
        Self { text }
    }

    pub fn execute(&self) -> Result<(), String> {
        println!("{}", self.text);
        Ok(())
    }
}
