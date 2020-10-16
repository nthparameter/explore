use crate::key_const::*;
use crate::window::EventHandler;
use std::iter;
use std::path::Path;

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
    row_ends: Vec<usize>,
    row_to_line: Vec<usize>,
    text: String,
    pub text_row_count: usize,
}

impl<'a> TextBuffer {
    pub fn new(file_path: &std::path::Path, data: String) -> Self {
        let text_row_count = data.lines().count();
        let mut tb = Self {
            file_path: file_path.to_path_buf(),
            lines: vec![],
            name: file_path.display().to_string(),
            pen_col: 0,
            pen_row: 0,
            rows: vec![],
            row_ends: vec![],
            row_to_line: vec![],
            text: data,
            text_row_count,
        };
        tb.parse_text();
        tb
    }

    pub fn get_row(&self, row: usize) -> Option<&str> {
        if row >= self.text_row_count {
            return None;
        }
        Some(&self.text[self.rows[row]..self.row_ends[row]])
    }

    pub fn get_row_width(&self, row: usize) -> Option<usize> {
        if row >= self.text_row_count {
            return None;
        }
        Some(self.row_ends[row] - self.rows[row])
    }

    fn insert_letter(&mut self, ch: char) {
        let offset = self.rows[self.pen_row] + self.pen_col;
        self.text = self.text[0..offset].to_string() + &ch.to_string() + &self.text[offset..];
        self.parse_text();
        self.on_cursor_right();
    }

    pub fn on_cursor_down(&mut self) {
        if self.pen_row + 1 < self.text_row_count {
            self.pen_row += 1;
            let row_len = self.get_row_width(self.pen_row).unwrap();
            if self.pen_col > row_len {
                self.pen_col = row_len;
            }
        }
    }

    pub fn on_cursor_left(&mut self) {
        if self.pen_col > 0 {
            self.pen_col -= 1;
        } else if self.pen_row > 0 {
            self.pen_row -= 1;
            self.pen_col = self.get_row_width(self.pen_row).unwrap();
        }
    }

    pub fn on_cursor_right(&mut self) {
        let row_limit = self.get_row_width(self.pen_row).unwrap();
        if self.pen_col < row_limit {
            self.pen_col += 1;
        } else if self.pen_row + 1 < self.text_row_count {
            self.pen_row += 1;
            self.pen_col = 0;
        }
    }

    pub fn on_cursor_up(&mut self) {
        if self.pen_row > 0 {
            self.pen_row -= 1;
            let row_len = self.get_row_width(self.pen_row).unwrap();
            if self.pen_col > row_len {
                self.pen_col = row_len;
            }
        }
    }

    fn parse_text(&mut self) {
        let mut row_len = 0;
        self.lines = vec![0];
        self.rows = vec![0];
        self.row_ends = vec![];
        self.row_to_line = vec![1];
        for (i, c) in self.text.chars().enumerate() {
            if c == '\n' {
                self.lines.push(self.rows.len());
                self.row_to_line.push(self.lines.len());
                self.row_ends.push(i);
                self.rows.push(i + 1);
                row_len = 0;
            } else {
                row_len += 1;
                if row_len > 40 {
                    // The 0 is a placeholder for line continuation.
                    self.row_to_line.push(0);
                    self.row_ends.push(i);
                    self.rows.push(i);
                    row_len = 0;
                }
            }
        }
        self.text_row_count = self.rows.len();
        // Push one extra entry to represent the last piece of text.
        self.row_ends.push(self.text.len());
    }

    pub fn line_numbers(&'a self) -> impl Iterator<Item = usize> + 'a {
        self.row_to_line.iter().cloned() //.into_iter()
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
                _ => {
                    match key_event.code {
                      crossterm::event::KeyCode::Char(ch) => self.insert_letter(ch),
                      _ => {}
                    }
                }
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
        TextBufferIterator { tb: self, index: 0 }
    }
}

impl<'a> Iterator for TextBufferIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        let result = self.tb.get_row(self.index);
        self.index += 1;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_buffer() {
        let tb = TextBuffer::new(Path::new("path/foo.txt"), "\npear\nmellon\n".to_string());
        assert_eq!(tb.file_path, Path::new("path/foo.txt"));
        assert_eq!(tb.lines, vec![0, 1, 2]);
        assert_eq!(tb.name, "path/foo.txt".to_string());
        assert_eq!(tb.pen_col, 0);
        assert_eq!(tb.pen_row, 0);
        //assert_eq!(tb.rows, vec!["pear".to_string(), "mellon".to_string());
        assert_eq!(tb.rows, vec![0, 1, 6, 13]);
        assert_eq!(tb.row_to_line, vec![1, 2, 3]);
        assert_eq!(tb.text, "\npear\nmellon\n".to_string());
        assert_eq!(tb.text_row_count, 3);

        assert_eq!(tb.get_row(0), Some("\n"));
        assert_eq!(tb.get_row(1), Some("pear\n"));
        assert_eq!(tb.get_row(2), Some("mellon\n"));
        assert_eq!(tb.get_row(3), None);

        let mut it = tb.line_numbers();
        assert_eq!(it.next(), Some(1));
        assert_eq!(it.next(), Some(2));
        assert_eq!(it.next(), Some(3));
        assert_eq!(it.next(), None);

        let mut it = tb.rows();
        assert_eq!(it.next(), Some("\n"));
        assert_eq!(it.next(), Some("pear\n"));
        assert_eq!(it.next(), Some("mellon\n"));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_text_buffer_long_line() {
        let text = "\nuse crate::buffer_manager::BufferManager;\nmellon\n".to_string();
        let tb = TextBuffer::new(Path::new("path/foo.txt"), text);
        assert_eq!(tb.lines, vec![0, 2, 3]);
        assert_eq!(tb.rows, vec![0, 1, 41, 43, 50]);
        assert_eq!(tb.row_to_line, vec![1, 0, 2, 3]);
        assert_eq!(tb.text_row_count, 4);

        assert_eq!(tb.get_row(0), Some("\n"));
        assert_eq!(
            tb.get_row(1),
            Some("use crate::buffer_manager::BufferManager")
        );
        assert_eq!(tb.get_row(2), Some(";\n"));
        assert_eq!(tb.get_row(3), Some("mellon\n"));
        assert_eq!(tb.get_row(4), None);

        let mut it = tb.line_numbers();
        assert_eq!(it.next(), Some(1));
        assert_eq!(it.next(), Some(0));
        assert_eq!(it.next(), Some(2));
        assert_eq!(it.next(), Some(3));
        assert_eq!(it.next(), None);

        let mut it = tb.rows();
        assert_eq!(it.next(), Some("\n"));
        assert_eq!(it.next(), Some("use crate::buffer_manager::BufferManager"));
        assert_eq!(it.next(), Some(";\n"));
        assert_eq!(it.next(), Some("mellon\n"));
        assert_eq!(it.next(), None);
    }
}
