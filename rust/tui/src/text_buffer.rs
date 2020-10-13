

use crate::key_const::*;
use crate::window::EventHandler;

pub struct TextBuffer {
    pub file_path: std::path::PathBuf,
    pub name: String,
    pub pen_col: usize,
    pub pen_row: usize,
    pub text: String,
    pub text_line_count: usize,
}

impl TextBuffer {

    pub fn new(file_path: &std::path::Path, data: String) -> Self {
        let text_line_count = data.lines().count();
        Self {
            file_path: file_path.to_path_buf(),
            name: file_path.display().to_string(),
            pen_col: 0,
            pen_row: 0,
            text: data,
            text_line_count,
        }
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

impl<'a> EventHandler for TextBuffer {
    fn handle_event(&mut self, event: &crossterm::event::Event) {
        if let crossterm::event::Event::Key(key_event) = event {
            match *key_event {
                KEY_DOWN => self.on_cursor_down(),
                KEY_LEFT => self.on_cursor_left(),
                KEY_RIGHT => self.on_cursor_right(),
                KEY_UP => self.on_cursor_up(),
                _ => {},
            }
        }
    }
}
