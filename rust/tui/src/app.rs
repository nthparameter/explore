
use crate::buffer_manager::BufferManager;
use crate::key_const::*;
use crate::text_window::TextWindow;
use crate::util::{TabsState};
use crate::window::EventHandler;
/*use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};*/

pub struct App<'a> {
    pub buffer_manager: BufferManager,
    pub title: &'a str,
    pub tabs: TabsState<'a>,
    pub progress: f64,
    pub enhanced_graphics: bool,
    pub pen_col: usize,
    pub pen_row: usize,
    pub scroll_left: usize,
    pub scroll_top: usize,
    pub debug_event: crossterm::event::Event,
    pub should_quit: bool,
    pub text_window: TextWindow<'a>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool) -> App<'a> {
        let mut buffer_manager = BufferManager::new();
        let mut text_window = TextWindow::new(
                    buffer_manager.new_text_buffer());

        App {
            buffer_manager,
            title,
            tabs: TabsState::new(vec!["Editor", "Terminal"]),
            progress: 0.0,
            enhanced_graphics,
            pen_col: 0,
            pen_row: 0,
            scroll_left: 0,
            scroll_top: 0,
            debug_event: crossterm::event::Event::Resize(1, 1),
            should_quit: false,
            text_window,
        }
    }
/*
    pub fn on_cursor_down(&mut self) {
        if self.pen_row < self.text_buffer.text_line_count {
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
    }*/
    pub fn on_open_file(&mut self) {
        let tb = self.buffer_manager.load(std::path::Path::new("src/app.rs"))
            .expect("read file");
        self.text_window
            .set_text_buffer(tb);
        //self.text = std::fs::read_to_string("src/app.rs").expect("read file");
        //self.text_line_count = self.text.lines().count();
    }
/*

    pub fn on_page_up(&mut self) {
        if self.scroll_top > 0 {
            self.scroll_top -= 1;
        }
    }

    pub fn on_page_down(&mut self) {
        if self.scroll_top < self.text_buffer.text_line_count {
            self.scroll_top += 1;
        }
    }

    pub fn on_scroll_up(&mut self) {
        if self.scroll_top > 0 {
            self.scroll_top -= 1;
        }
    }

    pub fn on_scroll_down(&mut self) {
        if self.scroll_top < self.text_buffer.text_line_count {
            self.scroll_top += 1;
        }
    }*/
/*
    pub fn on_right(&mut self) {
        self.tabs.next();
    }*/

    pub fn on_select_editor_tab(&mut self) {
        self.tabs.index = 0;
    }

    pub fn on_select_terminal_tab(&mut self) {
        self.tabs.index = 1;
    }
/*
    pub fn on_left(&mut self) {
        self.tabs.previous();
    }*/

    pub fn on_tick(&mut self) {
        // Update progress
        self.progress += 0.001;
        if self.progress > 1.0 {
            self.progress = 0.0;
        }
    }
}

impl<'a> EventHandler for App<'_> {
    fn handle_event(&mut self, event: &crossterm::event::Event) {
        if let crossterm::event::Event::Key(key_event) = event {
            match *key_event {
                CTRL_O => self.on_open_file(),
                CTRL_Q => self.should_quit = true,
                KEY_F2 => self.on_select_editor_tab(),
                KEY_F3 => self.on_select_terminal_tab(),
                _ => self.text_window.handle_event(event),
            }
        }
    }
}

