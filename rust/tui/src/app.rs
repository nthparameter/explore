//! Top level application model.

use crate::buffer_manager::BufferManager;
use crate::key_const::*;
use crate::text_window::TextWindow;
//use crate::program_window::ProgramWindow;
use crate::util::TabsState;
use crate::window::{EscalationEvent, Window};
use log;

/// A set of nested rectangles that maps a mouse click to the appropriate event
/// handler.
/*
struct ClickArea { name: string, area: Rect, handler: EventHandler, }
struct AreaHandler { areas: Vec<ClickArea>, }
impl AreaHandler {
    pub fn new() -> Self {
        Self {
            areas: vec!(),
        }
    }

    pub fn handler(x: i16, y: i16) -> Option<&ClickArea> {
        for area in areas {
            if area.contains(x, y) {
                return Some(area.handler);
            }
        }
        None
    }

    /// Add click area to the back, potentially being covered by existing areas.
    pub fn push_back(click_area: ClickArea) {
    }

    /// Add click area to the front, potentially covering existing areas.
    pub fn push_front(click_area: ClickArea) {
    }
}
*/

pub struct App<'a> {
    pub buffer_manager: BufferManager,
    pub title: &'a str,
    pub tabs: TabsState<'a>,
    pub progress: f64,
    pub debug_event: crossterm::event::Event,
    //pub open_file_view: OpenFileView,
    pub should_quit: bool,
    //pub program_window: ProgramWindow,
    pub text_window: TextWindow<'a>,
    //pub area_handler: AreaHandler,
    bounds: tui::layout::Rect,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, /*log: &mut Log,*/) -> Self {
        log::info!("Creating App");
        let mut buffer_manager = BufferManager::new();
        let text_window =
            TextWindow::new(buffer_manager.new_text_buffer(std::path::Path::new(&"")));
        //let mut open_file_view = OpenFileView::new(buffer_manager);

        let mut ts = TabsState::new(vec!["Help", "Open", "Search", "Edit", "Terminal"]);
        ts.index = 3;
        Self {
            buffer_manager,
            title,
            tabs: ts,
            progress: 0.0,
            debug_event: crossterm::event::Event::Resize(1, 1),
            should_quit: false,
            //program_window: ProgramWindow::new(&mut self),
            text_window,
            bounds: tui::layout::Rect::new(0, 0, 0, 0),
        }
    }

    pub fn close_file(&mut self) {}

    pub fn new_file(&mut self) {}

    pub fn open_file(&mut self, file_path: &std::path::Path) {
        self.text_window.set_text_buffer(match file_path.exists() {
            true => self.buffer_manager.load(file_path).expect("read file"),
            false => self.buffer_manager.new_text_buffer(file_path),
        });
    }

    pub fn save_all_files(&mut self) {
        self.buffer_manager.save_all_files();
    }

    pub fn on_select_editor_tab(&mut self) {
        log::info!("on_select_editor_tab");
        self.tabs.index = 3;
    }

    pub fn on_select_help_tab(&mut self) {
        log::info!("on_select_help_tab");
        self.tabs.index = 0;
    }

    pub fn on_select_open_tab(&mut self) {
        log::info!("on_select_open_tab");
        self.tabs.index = 1;
    }

    pub fn on_select_search_tab(&mut self) {
        log::info!("on_select_search_tab");
        self.tabs.index = 2;
    }

    pub fn on_select_terminal_tab(&mut self) {
        log::info!("on_select_terminal_tab");
        self.tabs.index = 4;
    }

    pub fn on_tick(&mut self) {
        // Update progress
        self.progress += 0.001;
        if self.progress > 1.0 {
            self.progress = 0.0;
        }
    }
}

impl<'a> Window for App<'_> {
    fn contains(&self, x: u16, y: u16) -> bool {
        tui::layout::Rect::new(x, y, 0, 0).intersects(self.bounds)
    }

    fn handle_event(&mut self, event: &crossterm::event::Event) -> EscalationEvent {
        self.debug_event = *event;
        match event {
            crossterm::event::Event::Key(key_event) => {
                log::info!("handle_event()");
                match self.text_window.handle_event(event) {
                    EscalationEvent::QuitProgram => self.should_quit = true,
                    EscalationEvent::Unhandled => {
                        log::info!("handle_event() escalation");
                        match *key_event {
                            CTRL_N => self.new_file(),
                            CTRL_O => self.on_select_open_tab(),
                            CTRL_S => self.save_all_files(),
                            CTRL_W => self.close_file(),
                            CTRL_Q => self.should_quit = true,
                            KEY_F1 => self.on_select_help_tab(),
                            KEY_F2 => self.on_select_open_tab(),
                            KEY_F3 => self.on_select_search_tab(),
                            KEY_F4 => self.on_select_editor_tab(),
                            KEY_F5 => self.on_select_terminal_tab(),
                            _ => return EscalationEvent::Unhandled,
                        }
                    }
                    _ => return EscalationEvent::Unhandled,
                }
            }
            crossterm::event::Event::Mouse(mouse_event) => {
                /*for each child, if child.contains(x,y) { child.handle_event(...); }*/
            }
            crossterm::event::Event::Resize(width, height) => {
                self.reshape(&tui::layout::Rect::new(0, 0, *width, *height));
            }
        }
        EscalationEvent::Handled
    }

    fn reshape(&mut self, rect: &tui::layout::Rect) {
        log::info!("reshape({:?})", rect);
        self.bounds = *rect;
    }
}
