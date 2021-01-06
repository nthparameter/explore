use crate::app::{App, AppFrame};
use crate::key_const::*;
use crate::text_buffer::TextBuffer;
use crate::window::{EscalationEvent, Window};
use std::sync::{Arc, Mutex};
use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Row, Wrap},
};
/*
#[derive(Debug, Clone)]
pub struct TextWindowState {
    scroll_top: usize,
    scroll_left: usize,
    focused: bool,
}

impl Default for TextWindowState {
    fn default() -> Self {
        Self {
            scroll_top: 0,
            scroll_left: 0,
            focused: false,
        }
    }
}*/

/// A window displaying (or editing) a text document (or more specifically a
/// TextBuffer).
pub struct TextWindow<'a> {
    pub block: Option<tui::widgets::Block<'a>>,
    pub focused: bool,
    pub render_height: usize,
    pub render_width: usize,
    pub scroll_top: usize,
    pub scroll_left: usize,
    pub text_buffer: Arc<Mutex<TextBuffer>>,
    bounds: tui::layout::Rect,
}

impl<'a> TextWindow<'a> {
    pub fn new(text_buffer: Arc<Mutex<TextBuffer>>) -> Self {
        Self {
            block: None,
            focused: false,
            render_height: 0,
            render_width: 0,
            scroll_top: 0,
            scroll_left: 0,
            text_buffer: text_buffer,
            bounds: tui::layout::Rect::new(0, 0, 0, 0),
        }
    }

    pub fn block(mut self, block: tui::widgets::Block<'a>) -> TextWindow<'a> {
        self.block = Some(block);
        self
    }

    pub fn on_page_down(&mut self) {
        let row_count = self.text_buffer.lock().unwrap().text_row_count;
        if row_count <= 2 * self.render_height + self.scroll_top {
            self.scroll_top = row_count - self.render_height;
        } else {
            self.scroll_top += self.render_height;
        }
    }

    pub fn on_page_up(&mut self) {
        if self.scroll_top > 0 {
            if self.scroll_top < self.render_height {
                self.scroll_top = 0;
            } else {
                self.scroll_top -= self.render_height;
            }
        }
    }

    pub fn on_scroll_down(&mut self) {
        if self.scroll_top < self.text_buffer.lock().unwrap().text_row_count {
            self.scroll_top += 1;
        }
    }

    pub fn on_scroll_up(&mut self) {
        if self.scroll_top > 0 {
            self.scroll_top -= 1;
        }
    }

    pub fn scroll_to_pen(&mut self) {
        let tb = self.text_buffer.lock().unwrap();
        if tb.pen_row < self.scroll_top {
            self.scroll_top = tb.pen_row;
        } else if tb.pen_row >= self.scroll_top + self.render_height {
            self.scroll_top = tb.pen_row - self.render_height + 1;
        }
        if tb.pen_col < self.scroll_left {
            self.scroll_left = tb.pen_col;
        } else if tb.pen_col >= self.scroll_left + self.render_width {
            self.scroll_left = tb.pen_col - self.render_width + 1;
        }
    }

    pub fn set_text_buffer(&mut self, tb: Arc<Mutex<TextBuffer>>) {
        self.text_buffer = tb;
    }

    /// Draw the body of the document (sans line numbers, etc.).
    fn draw_text(&self, frame: &mut AppFrame, area: tui::layout::Rect) {
        //log::info!("draw_text");
        let tb = self.text_buffer.lock().unwrap();
        let block = Block::default().borders(Borders::TOP).title(Span::styled(
            &tb.name,
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ));
        let inner_area = block.inner(area);
        let text = tb
            .rows()
            .skip(self.scroll_top)
            .take(area.height as usize)
            .map(|s| Spans::from(s))
            .collect::<Vec<Spans>>();
        let paragraph = Paragraph::new(text).block(block); //.wrap(Wrap { trim: false });
        frame.render_widget(paragraph, area);

        // Show the cursor if it's in the view.
        let height = inner_area.height as usize;
        let width = inner_area.width as usize;
        if self.scroll_top <= tb.pen_row
            && self.scroll_top + height > tb.pen_row
            && self.scroll_left <= tb.pen_col
            && self.scroll_left + width > tb.pen_col
        {
            frame.set_cursor(
                inner_area.x + (tb.pen_col - self.scroll_left) as u16,
                inner_area.y + (tb.pen_row - self.scroll_top) as u16,
            );
        }
    }

    /// Draw the line numbers (only).
    fn draw_line_numbers(&self, frame: &mut AppFrame, area: tui::layout::Rect) {
        let block = Block::default().borders(Borders::TOP);
        let text = self
            .text_buffer
            .lock()
            .unwrap()
            .line_numbers()
            .skip(self.scroll_top)
            .take(area.height as usize)
            .map(|s| {
                if s == 0 {
                    Spans::from("")
                } else {
                    Spans::from(s.to_string())
                }
            })
            .collect::<Vec<Spans>>();
        let paragraph = Paragraph::new(text).block(block);
        frame.render_widget(paragraph, area);
    }
}

impl<'a> Window for TextWindow<'a> {
    fn contains(&self, x: u16, y: u16) -> bool {
        tui::layout::Rect::new(x, y, 0, 0).intersects(self.bounds)
    }

    fn draw(&self, frame: &mut AppFrame) {
        let h_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(7), Constraint::Min(0)].as_ref())
            .split(self.bounds);
        self.draw_line_numbers(frame, h_chunks[0]);
        self.draw_text(frame, h_chunks[1]);
    }

    fn handle_event(&mut self, event: &crossterm::event::Event) -> EscalationEvent {
        if let crossterm::event::Event::Key(key_event) = event {
            match *key_event {
                CTRL_DOWN => self.on_scroll_down(),
                CTRL_UP => self.on_scroll_up(),
                KEY_PAGE_DOWN => self.on_page_down(),
                KEY_PAGE_UP => self.on_page_up(),
                _ => {
                    {
                        let tb = &mut self.text_buffer.lock().unwrap();
                        match *key_event {
                            CTRL_END => tb.pen_bottom(),
                            CTRL_HOME => tb.pen_top(),
                            CTRL_C => tb.copy_selection(),
                            CTRL_V => tb.paste(),
                            CTRL_X => tb.cut_selection(),
                            CTRL_Y => tb.redo(),
                            CTRL_Z => tb.undo(),
                            KEY_BACKSPACE => tb.backspace(),
                            KEY_DELETE => tb.delete(),
                            KEY_DOWN => tb.pen_down_or_end(),
                            KEY_END => tb.pen_row_end(),
                            KEY_ENTER => tb.carriage_return(),
                            KEY_HOME => tb.pen_row_start(),
                            KEY_LEFT => tb.pen_left(),
                            KEY_RIGHT => tb.pen_right(),
                            KEY_UP => tb.pen_up_or_start(),
                            crossterm::event::KeyEvent {
                                code: ch,
                                modifiers: crossterm::event::KeyModifiers::CONTROL,
                            } => return EscalationEvent::Unhandled,
                            crossterm::event::KeyEvent {
                                code: ch,
                                modifiers: crossterm::event::KeyModifiers::ALT,
                            } => return EscalationEvent::Unhandled,
                            _ => match key_event.code {
                                crossterm::event::KeyCode::Char(ch) => tb.insert_letter(ch),
                                _ => return EscalationEvent::Unhandled,
                            },
                        }
                    }
                    self.scroll_to_pen();
                }
            }
        }
        EscalationEvent::Handled
    }

    fn reshape(&mut self, rect: &tui::layout::Rect) {
        log::info!("reshape {:?}", rect);
        self.bounds = *rect;
        //self.render_width = inner_area.width as usize;
        //self.render_height = inner_area.height as usize;
        self.render_width = self.bounds.width as usize;
        self.render_height = self.bounds.height as usize - 1;
    }
}
