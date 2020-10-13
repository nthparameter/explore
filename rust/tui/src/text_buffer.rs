

use crate::key_const::*;
use crate::window::EventHandler;
use std::iter;

pub struct TextBuffer {
    pub file_path: std::path::PathBuf,
    pub name: String,
    pub pen_col: usize,
    pub pen_row: usize,
    rows: Vec<usize>,
    text: String,
    pub text_line_count: usize,
}


impl TextBuffer {

    pub fn new(file_path: &std::path::Path, data: String) -> Self {
        let text_line_count = data.lines().count();
        let mut tb = Self {
            file_path: file_path.to_path_buf(),
            name: file_path.display().to_string(),
            pen_col: 0,
            pen_row: 0,
            rows: vec![],
            text: data,
            text_line_count,
        };
        tb.parse_text();
        tb
    }

    pub fn get_row(&self, row: usize) -> Option<&str> {
        if row >= self.text_line_count {
            return None;
        }
        Some(&self.text[self.rows[row]..self.rows[row+1]])
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

    fn parse_text(&mut self) {
        let mut line_start = 0;
        let mut row_len = 0;
        self.rows.push(line_start);
        for (i,c) in self.text.chars().enumerate() {
            if c == '\n' {
                self.rows.push(line_start);
                line_start = i + 1;
                row_len = 0;
            } else {
                row_len += 1;
                if row_len > 40 {
                    self.rows.push(line_start);
                    line_start = i;
                    row_len = 0;
                }
            }
        }
    }

    pub fn rows(&self) -> impl Iterator<Item = &str> {
        self.into_iter()
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

pub struct TextBufferIterator<'a> {
    tb: &'a TextBuffer,
    index: usize,
}

impl<'a> IntoIterator for &'a TextBuffer {
    type Item = &'a str;
    type IntoIter = TextBufferIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TextBufferIterator {
            tb: self,
            index: 0,
        }
    }
}


impl<'a> Iterator for TextBufferIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        self.index += 1;
        self.tb.get_row(self.index)
        /*
        if self.index >= self.tb.text_line_count {
            return None;
        }
        Some(self.tb.get_row(self.index).unwrap())*/
    }
}
