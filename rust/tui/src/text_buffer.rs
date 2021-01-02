use crate::key_const::*;
//use crate::window::{EscalationEvent, EventHandler};
use log;
use more_asserts::*;
use std::iter;
use std::path::Path;

pub enum Selection {
    Char {
        col: usize,
        row: usize,
        chars: usize,
    },
    Word {
        col: usize,
        row: usize,
        chars: usize,
    },
    // Rectangular, each row starts at same char width column.
    Rect {
        col: usize,
        row: usize,
        forward: usize,
        down: usize,
    },
    Line {
        row: usize,
        down: usize,
    },
    // End-of-line, each row starts at '\n' or end-of-file.
    Eol {
        row: usize,
        down: usize,
    },
}

pub struct Transform {
    shape: Selection,
    old: String,
    new: String,
}

pub struct TextBuffer {
    /// The OS specific path to the file backing the buffer.
    pub file_path: std::path::PathBuf,
    /// Each entry in lines is an index into `rows` for the first row of the
    /// line.
    lines: Vec<usize>,
    /// String version of the file_path or placeholder for new buffers that are
    /// not backed by a file on disk.
    pub name: String,
    /// The column where edits will occur.
    pub pen_col: usize,
    /// the row where edits will occur.
    pub pen_row: usize,
    /// Each entry in rows is an offset to a character in `text`.
    rows: Vec<usize>,
    /// Index of '\n' characters (or end of file) in 'text'.
    row_ends: Vec<usize>,
    /// Convert a row index into a line number. In wrapped lines the row index
    /// may be much higher than the line number (as each line may be any number
    /// of rows).
    row_to_line: Vec<usize>,
    /// The raw contents of the buffer.
    text: String,
    pub text_row_count: usize,
    transform_index: usize,
    transforms: Vec<Transform>,
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
            transform_index: 0,
            transforms: vec![],
        };
        tb.parse_text();
        tb
    }

    pub fn apply_transform(&mut self) -> (usize, usize) {
        debug_assert!(self.transform_index <= self.transforms.len());
        if self.transform_index == self.transforms.len() {
            return (0, 0);
        }
        let transform = &self.transforms[self.transform_index];
        let mut rec_col = 0;
        let mut rec_row = 0;
        match transform.shape {
            Selection::Char { col, row, chars } => {
                let offset = self.rows[row] + col;
                self.text = self.text[0..offset].to_string()
                    + &transform.new
                    + &self.text[offset + transform.old.len()..];
                if let Some(last_cr) = transform.new.rfind("\n") {
                    rec_row = row + transform.new.split("\n").count() - 1;
                    rec_col = transform.new.len() - (last_cr + 1);
                } else {
                    rec_col = col + transform.new.len();
                }
            }
            Selection::Word { col, row, chars } => (),
            Selection::Rect {
                col,
                row,
                forward,
                down,
            } => (),
            Selection::Line { row, down } => (),
            Selection::Eol { row, down } => (),
        }
        self.transform_index += 1;
        self.parse_text();
        (rec_col, rec_row)
    }

    pub fn remove_transform(&mut self) -> (usize, usize) {
        if self.transform_index == 0 {
            return (0, 0);
        }
        self.transform_index -= 1;
        let transform = &self.transforms[self.transform_index];
        let mut rec_col = 0;
        let mut rec_row = 0;
        match transform.shape {
            Selection::Char { col, row, chars } => {
                let offset = self.rows[row] + col;
                self.text = self.text[0..offset].to_string()
                    + &transform.old
                    + &self.text[offset + transform.new.len()..];
                rec_row = row;
                rec_col = col;
            }
            Selection::Word { col, row, chars } => (),
            Selection::Rect {
                col,
                row,
                forward,
                down,
            } => (),
            Selection::Line { row, down } => (),
            Selection::Eol { row, down } => (),
        }
        self.parse_text();
        (rec_col, rec_row)
    }

    pub fn carriage_return(&mut self) {
        let shape = Selection::Char {
            col: self.pen_col,
            row: self.pen_row,
            chars: 0,
        };
        let old = self.yank(&shape);
        self.transforms.push(Transform {
            shape,
            old,
            new: "\n".to_string(),
        });
        let (col, row) = self.apply_transform();
        self.pen_col = col;
        self.pen_row = row;
        /*
        let offset = self.rows[self.pen_row] + self.pen_col;
        self.text = self.text[0..offset].to_string() + &"\n" + &self.text[offset..];
        self.parse_text();
        self.pen_right();
        */
    }

    pub fn yank(&self, shape: &Selection) -> String {
        match shape {
            Selection::Char { col, row, chars } => {
                let offset = self.rows[*row] + col;
                self.text[offset..offset + chars].to_string()
            }
            Selection::Word { col, row, chars } => {
                let offset = self.rows[*row] + col;
                self.text[offset..offset + chars].to_string()
            }
            Selection::Rect {
                col,
                row,
                forward,
                down,
            } => {
                let offset = self.rows[*row] + col;
                self.text[offset..offset + forward].to_string()
            }
            Selection::Line { row, down } => {
                let offset = self.rows[*row];
                let end = self.rows[row + down];
                self.text[offset..end].to_string()
            }
            Selection::Eol { row, down } => "".to_string(),
        }
    }

    pub fn copy_selection(&mut self) {}

    pub fn cut_selection(&mut self) {}

    pub fn paste(&mut self) {}

    /// Get the contents of a row of the buffer. Excludes the trailing newline.
    pub fn get_row(&self, row: usize) -> Option<&str> {
        if row >= self.text_row_count {
            return None;
        }
        Some(&self.text[self.rows[row]..self.row_ends[row]])
    }

    /// The width of the row in display cells (some characters are zero width,
    /// while others may be double width).
    pub fn get_row_width(&self, row: usize) -> Option<usize> {
        if row >= self.text_row_count {
            return None;
        }
        debug_assert_eq!(self.rows.len(), self.row_ends.len());
        debug_assert_lt!(row, self.row_ends.len());
        debug_assert_ge!(self.row_ends[row], self.rows[row]);
        Some(self.row_ends[row] - self.rows[row])
    }

    /// Add a character to the buffer at the pen row/col location.
    pub fn insert_letter(&mut self, ch: char) {
        let offset = self.rows[self.pen_row] + self.pen_col;
        self.text = self.text[0..offset].to_string() + &ch.to_string() + &self.text[offset..];
        self.parse_text();
        self.pen_right();
    }

    pub fn line_numbers(&'a self) -> impl Iterator<Item = usize> + 'a {
        self.row_to_line.iter().cloned() //.into_iter()
    }

    /// Move pen to bottom of document.
    pub fn pen_bottom(&mut self) {
        self.pen_row = self.text_row_count - 1;
        self.pen_col = self.get_row_width(self.pen_row).unwrap();
    }

    /// Move the pen down one row, or if pen is on the last row move to the
    /// end of that row.
    pub fn pen_down_or_end(&mut self) {
        if self.pen_row + 1 < self.text_row_count {
            self.pen_row += 1;
            let row_len = self.get_row_width(self.pen_row).unwrap();
            if self.pen_col > row_len {
                self.pen_col = row_len;
            }
        } else {
            assert_eq!(self.pen_row, self.text_row_count - 1);
            self.pen_col = self.get_row_width(self.pen_row).unwrap();
        }
    }

    /// Move pen to end of current row.
    pub fn pen_row_end(&mut self) {
        self.pen_col = self.get_row_width(self.pen_row).unwrap();
    }

    /// Move pen to start of current row.
    pub fn pen_row_start(&mut self) {
        self.pen_col = 0;
    }

    /// Move the edit location one character to the left, wrapping to end of the
    /// prior row if appropriate.
    pub fn pen_left(&mut self) {
        if self.pen_col > 0 {
            self.pen_col -= 1;
        } else if self.pen_row > 0 {
            self.pen_row -= 1;
            self.pen_col = self.get_row_width(self.pen_row).unwrap();
        }
    }

    /// Move the edit location one character to the right, wrapping to start of
    /// the next row if appropriate.
    pub fn pen_right(&mut self) {
        let row_limit = self.get_row_width(self.pen_row).unwrap();
        if self.pen_col < row_limit {
            self.pen_col += 1;
        } else if self.pen_row + 1 < self.text_row_count {
            self.pen_row += 1;
            self.pen_col = 0;
        }
    }

    /// Move pen up one row, or if at the top row move to start of row.
    pub fn pen_up_or_start(&mut self) {
        if self.pen_row > 0 {
            self.pen_row -= 1;
            let row_len = self.get_row_width(self.pen_row).unwrap();
            if self.pen_col > row_len {
                self.pen_col = row_len;
            }
        } else {
            self.pen_col = 0;
        }
    }

    /// Move pen to bottom of document.
    pub fn pen_top(&mut self) {
        self.pen_row = 0;
        self.pen_col = 0;
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

        // Post conditions.
        debug_assert_eq!(self.text_row_count, self.rows.len());
        debug_assert_eq!(self.rows.len(), self.row_ends.len());
        //debug_assert_lt!(self.text_row_count, self.row_ends.len());
        //debug_assert_le!(self.lines.len(), self.rows.len());
    }

    pub fn redo(&mut self) {
        let (col, row) = self.apply_transform();
        self.pen_col = col;
        self.pen_row = row;
    }

    pub fn rows(&self) -> impl Iterator<Item = &str> {
        self.into_iter()
    }

    pub fn undo(&mut self) {
        let (col, row) = self.remove_transform();
        self.pen_col = col;
        self.pen_row = row;
    }

    pub fn text_bytes(&self) -> &[u8] {
        &self.text.as_bytes()
    }

    /*
        pub fn text_iter(&self) -> impl Iterator<Item = u8> {
            self.text.into_bytes().into_iter()
        }
    */
}
/*
impl<'a> Window for TextBuffer {
    fn handle_event(&mut self, event: &crossterm::event::Event) -> EscalationEvent {
        log::info!("handle_event");
        if let crossterm::event::Event::Key(key_event) = event {
            match *key_event {
                CTRL_END => self.pen_bottom(),
                CTRL_HOME => self.pen_top(),
                CTRL_C => self.copy_selection(),
                CTRL_V => self.paste(),
                CTRL_X => self.cut_selection(),
                CTRL_Y => self.redo(),
                CTRL_Z => self.undo(),
                KEY_DOWN => self.pen_down_or_end(),
                KEY_END => self.pen_row_end(),
                KEY_ENTER => self.carriage_return(),
                KEY_HOME => self.pen_row_start(),
                KEY_LEFT => self.pen_left(),
                KEY_RIGHT => self.pen_right(),
                KEY_UP => self.pen_up_or_start(),
                crossterm::event::KeyEvent {
                    code: ch,
                    modifiers: crossterm::event::KeyModifiers::CONTROL,
                } => return EscalationEvent::Unhandled,
                crossterm::event::KeyEvent {
                    code: ch,
                    modifiers: crossterm::event::KeyModifiers::ALT,
                } => return EscalationEvent::Unhandled,
                _ => match key_event.code {
                    crossterm::event::KeyCode::Char(ch) => self.insert_letter(ch),
                    _ => return EscalationEvent::Unhandled,
                },
            }
        }
        log::info!("true");
        return EscalationEvent::Handled;
    }
}*/

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
        assert_eq!(tb.lines, vec![0, 1, 2, 3]);
        assert_eq!(tb.name, "path/foo.txt".to_string());
        assert_eq!(tb.pen_col, 0);
        assert_eq!(tb.pen_row, 0);
        //assert_eq!(tb.rows, vec!["pear".to_string(), "mellon".to_string());
        assert_eq!(tb.rows, &[0, 1, 6, 13]);
        assert_eq!(tb.row_ends, &[0, 5, 12, 13]);
        assert_eq!(tb.row_to_line, &[1, 2, 3, 4]);
        assert_eq!(tb.text, "\npear\nmellon\n".to_string());
        assert_eq!(tb.text_row_count, 4);

        assert_eq!(tb.get_row(0), Some(""));
        assert_eq!(tb.get_row(1), Some("pear"));
        assert_eq!(tb.get_row(2), Some("mellon"));
        assert_eq!(tb.get_row(3), Some(""));
        assert_eq!(tb.get_row(4), None);

        let mut it = tb.line_numbers();
        assert_eq!(it.next(), Some(1));
        assert_eq!(it.next(), Some(2));
        assert_eq!(it.next(), Some(3));
        assert_eq!(it.next(), Some(4));
        assert_eq!(it.next(), None);

        let mut it = tb.rows();
        assert_eq!(it.next(), Some(""));
        assert_eq!(it.next(), Some("pear"));
        assert_eq!(it.next(), Some("mellon"));
        assert_eq!(it.next(), Some(""));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_text_buffer_long_line() {
        let text = "\nuse crate::buffer_manager::BufferManager;\nmellon\n".to_string();
        let mut tb = TextBuffer::new(Path::new("path/foo.txt"), text);
        assert_eq!(tb.lines, vec![0, 1, 3, 4]);
        assert_eq!(tb.rows, vec![0, 1, 41, 43, 50]);
        assert_eq!(tb.row_ends, &[0, 41, 42, 49, 50]);
        assert_eq!(tb.row_to_line, vec![1, 2, 0, 3, 4]);
        assert_eq!(tb.text_row_count, 5);

        assert_eq!(tb.get_row(0), Some(""));
        assert_eq!(
            tb.get_row(1),
            Some("use crate::buffer_manager::BufferManager")
        );
        assert_eq!(tb.get_row(2), Some(";"));
        assert_eq!(tb.get_row(3), Some("mellon"));
        assert_eq!(tb.get_row(4), Some(""));
        assert_eq!(tb.get_row(5), None);

        let mut it = tb.line_numbers();
        assert_eq!(it.next(), Some(1));
        assert_eq!(it.next(), Some(2));
        assert_eq!(it.next(), Some(0));
        assert_eq!(it.next(), Some(3));
        assert_eq!(it.next(), Some(4));
        assert_eq!(it.next(), None);
        drop(it);

        let mut it = tb.rows();
        assert_eq!(it.next(), Some(""));
        assert_eq!(it.next(), Some("use crate::buffer_manager::BufferManager"));
        assert_eq!(it.next(), Some(";"));
        assert_eq!(it.next(), Some("mellon"));
        assert_eq!(it.next(), Some(""));
        assert_eq!(it.next(), None);
        drop(it);

        assert_eq!(tb.pen_row, 0);
        tb.pen_down_or_end();
        assert_eq!(tb.pen_row, 1);
        tb.pen_down_or_end();
        assert_eq!(tb.pen_row, 2);
        tb.pen_down_or_end();
        assert_eq!(tb.pen_row, 3);
        tb.pen_down_or_end();
        assert_eq!(tb.pen_row, 4);
        assert_eq!(tb.pen_col, 0);
        tb.pen_down_or_end();
        assert_eq!(tb.pen_row, 4);
        assert_eq!(tb.pen_col, 0);
    }

    #[test]
    fn test_one_line_no_eol() {
        let text = "short".to_string();
        let mut tb = TextBuffer::new(Path::new("path/foo.txt"), text);
        assert_eq!(tb.lines, vec![0]);
        assert_eq!(tb.rows, vec![0]);
        assert_eq!(tb.row_ends, &[5]);
        assert_eq!(tb.row_to_line, vec![1]);
        assert_eq!(tb.text_row_count, 1);

        assert_eq!(tb.get_row(0), Some("short"));
        assert_eq!(tb.get_row(1), None);

        let mut it = tb.line_numbers();
        assert_eq!(it.next(), Some(1));
        assert_eq!(it.next(), None);
        drop(it);

        let mut it = tb.rows();
        assert_eq!(it.next(), Some("short"));
        assert_eq!(it.next(), None);
        drop(it);

        assert_eq!(tb.get_row_width(0), Some(5));
        assert_eq!(tb.get_row_width(1), None);

        assert_eq!(tb.pen_row, 0);
        assert_eq!(tb.pen_col, 0);
        tb.pen_down_or_end();
        assert_eq!(tb.pen_row, 0);
        assert_eq!(tb.pen_col, 5);
        tb.pen_down_or_end();
        assert_eq!(tb.pen_row, 0);
        assert_eq!(tb.pen_col, 5);
    }
}
