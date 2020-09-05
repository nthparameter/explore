pub struct Buffer {
    pub name: String,
    pub file_path: String,
    pub text: String,
    pub text_line_count: usize,
}

impl Buffer {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            file_path: "".to_string(),
            text: "".to_string(),
            text_line_count: 0,
        }
    }

    pub fn load(&mut self, file_path: &std::path::Path) -> std::io::Result<()> {
        self.text = std::fs::read_to_string("src/app.rs")?;
        self.text_line_count = self.text.lines().count();
        Ok(())
    }
}
