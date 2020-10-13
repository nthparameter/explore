
use crate::key_const::*;
use crate::text_buffer::TextBuffer;
use crate::window::EventHandler;
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
        }
    }

    pub fn block(mut self, block: tui::widgets::Block<'a>) -> TextWindow<'a> {
        self.block = Some(block);
        self
    }

    pub fn on_page_down(&mut self) {
        let line_count = self.text_buffer.lock().unwrap().text_line_count;
        if line_count <= 2 * self.render_height + self.scroll_top  {
            self.scroll_top = line_count - self.render_height;
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
        if self.scroll_top < self.text_buffer.lock().unwrap().text_line_count {
            self.scroll_top += 1;
        }
    }

    pub fn on_scroll_up(&mut self) {
        if self.scroll_top > 0 {
            self.scroll_top -= 1;
        }
    }

    pub fn set_text_buffer(&mut self, tb: Arc<Mutex<TextBuffer>>) {
        self.text_buffer = tb;
    }
}

impl<'a> EventHandler for TextWindow<'a> {
    fn handle_event(&mut self, event: &crossterm::event::Event) {
        if let crossterm::event::Event::Key(key_event) = event {
            match *key_event {
                CTRL_DOWN => self.on_scroll_down(),
                CTRL_UP => self.on_scroll_up(),
                KEY_PAGE_DOWN => self.on_page_down(),
                KEY_PAGE_UP => self.on_page_up(),
                _ => self.text_buffer.lock().unwrap().handle_event(event),
            }
        }
    }
}
