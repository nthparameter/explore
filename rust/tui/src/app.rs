use crate::buffer;
use crate::util::{RandomSignal, SinSignal, StatefulList, TabsState};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
/*
const TASKS: [&str; 24] = [
    "Item1", "Item2", "Item3", "Item4", "Item5", "Item6", "Item7", "Item8", "Item9", "Item10",
    "Item11", "Item12", "Item13", "Item14", "Item15", "Item16", "Item17", "Item18", "Item19",
    "Item20", "Item21", "Item22", "Item23", "Item24",
];*/

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    //pub text: String,
    //pub text_line_count: usize,
    //pub show_chart: bool,
    pub progress: f64,
    pub enhanced_graphics: bool,
    pub pen_col: usize,
    pub pen_row: usize,
    pub scroll_left: usize,
    pub scroll_top: usize,
    pub debug_event: crossterm::event::Event,
    pub buffer: buffer::Buffer,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool) -> App<'a> {
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["Editor", "Terminal"]),
            //text: String::new(),
            //text_line_count: 0,
            //show_chart: true,
            progress: 0.0,
            enhanced_graphics,
            pen_col: 0,
            pen_row: 0,
            scroll_left: 0,
            scroll_top: 0,
            debug_event: crossterm::event::Event::Resize(1, 1),
            buffer: buffer::Buffer::new("<none>".to_string()),
        }
    }

    pub fn on_cursor_down(&mut self) {
        if self.pen_row < self.buffer.text_line_count {
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

    pub fn on_open_file(&mut self) {
        self.buffer
            .load(std::path::Path::new("src/app.rs"))
            .expect("read file");
        //self.text = std::fs::read_to_string("src/app.rs").expect("read file");
        //self.text_line_count = self.text.lines().count();
    }

    pub fn on_page_up(&mut self) {
        if self.scroll_top > 0 {
            self.scroll_top -= 1;
        }
    }

    pub fn on_page_down(&mut self) {
        if self.scroll_top < self.buffer.text_line_count {
            self.scroll_top += 1;
        }
    }

    pub fn on_scroll_up(&mut self) {
        if self.scroll_top > 0 {
            self.scroll_top -= 1;
        }
    }

    pub fn on_scroll_down(&mut self) {
        if self.scroll_top < self.buffer.text_line_count {
            self.scroll_top += 1;
        }
    }

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_select_editor_tab(&mut self) {
        self.tabs.index = 0;
    }

    pub fn on_select_terminal_tab(&mut self) {
        self.tabs.index = 1;
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }
    /*
        pub fn on_key(&mut self, c: char) {
            match c {
                'q' => {
                    self.should_quit = true;
                }
                't' => {
                    self.show_chart = !self.show_chart;
                }
                _ => {}
            }
        }
    */
    pub fn on_tick(&mut self) {
        // Update progress
        self.progress += 0.001;
        if self.progress > 1.0 {
            self.progress = 0.0;
        }
    }
}
