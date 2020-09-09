

use crate::window::EventHandler;

pub struct TextBuffer {
    pub name: String,
    pub file_path: std::path::PathBuf,
    pub pen_col: usize,
    pub pen_row: usize,
    pub text: String,
    pub text_line_count: usize,
}

impl TextBuffer {
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

    pub fn on_cursor_down(&mut self) {
        if self.pen_row < self.text_line_count {
            self.pen_row += 1;
        }
    }

    pub fn on_cursor_left(&mut self) {
        if self.pen_col > 0 {
            self.pen_col -= 1;
        }
    }

    pub fn on_cursor_right(&mut self) {
        if self.pen_col < 10 {
            self.pen_col += 1;
        }
    }

    pub fn on_cursor_up(&mut self) {
        if self.pen_row > 0 {
            self.pen_row -= 1;
        }
    }
}

impl<'a> EventHandler for TextBuffer<'a> {
    fn handle_event(&mut self, event: &crossterm::event::Event) {
        if let crossterm::event::Event::Key(key_event) = event {
            match key_event {
                KEY_DOWN => self.on_cursor_down(),
                KEY_LEFT => self.on_cursor_left(),
                KEY_RIGHT => self.on_cursor_right(),
                KEY_UP => self.on_cursor_up(),
                _ => {},
            }
        }
    }
}
