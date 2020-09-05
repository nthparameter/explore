pub struct Buffer {
    pub name: String,
    pub file_path: std::path::PathBuf,
    pub text: String,
    pub text_line_count: usize,
}

impl Buffer {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            file_path: std::path::PathBuf::new(),
            text: "".to_string(),
            text_line_count: 0,
        }
    }

    pub fn load(&mut self, file_path: &std::path::Path) -> std::io::Result<()> {
        self.name = file_path.display().to_string();
        self.file_path = file_path.to_path_buf();
        self.text = std::fs::read_to_string("src/app.rs")?;
        self.text_line_count = self.text.lines().count();
        Ok(())
    }
}
