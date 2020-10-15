

use crate::key_const::*;
use crate::window::EventHandler;
use std::iter;

pub struct TextBuffer {
    pub file_path: std::path::PathBuf,
    // Each entry in lines is an index into `rows` for the first row of the
    // line.
    lines: Vec<usize>,
    pub name: String,
    pub pen_col: usize,
    pub pen_row: usize,
    // Each entry in rows is an offset to a character in `text`.
    rows: Vec<usize>,
    row_to_line: Vec<usize>,
    text: String,
    pub text_line_count: usize,
}


impl<'a> TextBuffer {

    pub fn new(file_path: &std::path::Path, data: String) -> Self {
        let text_line_count = data.lines().count();
        let mut tb = Self {
            file_path: file_path.to_path_buf(),
            lines: vec![],
            name: file_path.display().to_string(),
            pen_col: 0,
            pen_row: 0,
            rows: vec![],
            row_to_line: vec![],
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
        self.lines = vec![0];
        self.rows = vec![line_start];
        self.row_to_line = vec![1];
        for (i,c) in self.text.chars().enumerate() {
            if c == '\n' {
                self.lines.push(self.rows.len());
                self.row_to_line.push(self.lines.len());
                self.rows.push(line_start);
                line_start = i + 1;
                row_len = 0;
            } else {
                row_len += 1;
                if row_len > 40 {
                    // The 0 is a placeholder for line continuation.
                    self.row_to_line.push(0);
                    self.rows.push(line_start);
                    line_start = i;
                    row_len = 0;
                }
            }
        }
        self.text_line_count = self.rows.len();
        // Push one extra entry to represent the last piece of text.
        self.rows.push(self.text.len());
    }

    pub fn line_numbers(&'a self) -> impl Iterator<Item = usize> + 'a {
        self.row_to_line.iter().cloned()//.into_iter()
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
    }
}
