use crate::key_const::*;
use crate::text_buffer::TextBuffer;
use crate::window::{EscalationEvent, Window};
use std::sync::{Arc, Mutex};
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
}

impl<'a> Window for TextWindow<'a> {
    fn contains(&self, x: u16, y: u16) -> bool {
        tui::layout::Rect::new(x, y, 0, 0).intersects(self.bounds)
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
        self.bounds = *rect;
    }
}
