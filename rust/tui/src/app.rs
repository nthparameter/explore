//! Top level application model.

use crate::buffer_manager::BufferManager;
use crate::color_window::ColorWindow;
use crate::debug_window::DebugWindow;
use crate::file_manager_window::FileManagerWindow;
use crate::help_window::HelpWindow;
use crate::key_const::*;
use crate::log_window::LogWindow;
use crate::search_window::SearchWindow;
use crate::terminal_window::TerminalWindow;
use crate::tabs_window::TabsWindow;
use crate::text_window::TextWindow;
//use crate::program_window::ProgramWindow;
use crate::util::TabsState;
use crate::window::{EscalationEvent, Window};
use log;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Row, Tabs, Wrap},
};

pub type AppFrame<'a> = tui::Frame<'a, CrosstermBackend<std::io::Stdout>>;

pub struct App<'a> {
    bounds: tui::layout::Rect,
    pub buffer_manager: BufferManager,
    child: Vec<Box<Window>>,
    pub color_window: ColorWindow,
    pub debug_event: crossterm::event::Event,
    pub debug_window: DebugWindow,
    pub file_manager_window: FileManagerWindow,
    pub help_window: HelpWindow,
    pub log_window: LogWindow,
    pub progress: f64,
    //pub open_file_view: OpenFileView,
    pub search_window: SearchWindow,
    pub should_quit: bool,
    //pub program_window: ProgramWindow,
    pub tabs_window: TabsWindow,
    pub terminal_window: TerminalWindow,
    pub text_window: Box<TextWindow<'a>>,
    //pub area_handler: AreaHandler,
    pub title: &'a str,
    pub tabs: TabsState<'a>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> Self {
        log::info!("Creating App");
        let mut buffer_manager = BufferManager::new();
        let terminal_window = Box::new(TextWindow::new(
            buffer_manager.new_text_buffer(std::path::Path::new(&"")),
        ));
        let text_window = Box::new(TextWindow::new(
            buffer_manager.new_text_buffer(std::path::Path::new(&"")),
        ));
        //let mut open_file_view = OpenFileView::new(buffer_manager);

        let mut ts = TabsState::new(vec!["Help", "Open", "Search", "Edit", "Terminal"]);
        ts.index = 3;
        Self {
            bounds: tui::layout::Rect::new(0, 0, 0, 0),
            buffer_manager,
            child: vec![],
            color_window: ColorWindow::default(),
            debug_event: crossterm::event::Event::Resize(1, 1),
            debug_window: DebugWindow::default(),
            file_manager_window: FileManagerWindow::default(),
            help_window: HelpWindow::default(),
            log_window: LogWindow::default(),
            progress: 0.0,
            search_window: SearchWindow::default(),
            should_quit: false,
            tabs_window: TabsWindow::default(),
            terminal_window: TerminalWindow::default(),
            title,
            tabs: ts,
            //program_window: ProgramWindow::new(&mut self),
            text_window,
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

    pub fn on_select_color_tab(&mut self) {
        log::info!("on_select_color_tab");
        self.tabs.index = 5;
    }

    pub fn on_tick(&mut self) {
        self.debug_window.tick_count += 1;
        /*// Update progress
        self.progress += 0.001;
        if self.progress > 1.0 {
            self.progress = 0.0;
        }*/
    }
}

impl<'a> Window for App<'_> {
    fn contains(&self, x: u16, y: u16) -> bool {
        tui::layout::Rect::new(x, y, 0, 0).intersects(self.bounds)
    }

    /// Root draw call for the application. This will call draw() on all needed
    /// nested UI elements.
    fn draw(&self, frame: &mut AppFrame /*, app: &mut App*/) {
        // Create UI tabs (labels for tabs).
        /*let tab_titles = app
            .tabs
            .titles
            .iter()
            .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Green))))
            .collect();
        let tabs = Tabs::new(tab_titles)
            .block(Block::default().borders(Borders::BOTTOM).title(app.title))
            .highlight_style(Style::default().fg(Color::Yellow))
            .select(app.tabs.index);*/
        /*self.tabs_window.draw(frame);*/
        // Only render the visible (selected) tab.
        //self.child[self.tabs.index].draw(frame);
        match self.tabs.index {
            0 => self.help_window.draw(frame),
            1 => self.file_manager_window.draw(frame),
            2 => self.search_window.draw(frame),
            3 => self.text_window.draw(frame),
            4 => self.terminal_window.draw(frame),
            5 => self.color_window.draw(frame),
            _ => {}
        };
        self.debug_window.draw(frame);
        self.log_window.draw(frame);
    }

    fn handle_event(&mut self, event: &crossterm::event::Event) -> EscalationEvent {
        self.debug_window.debug_event = *event;
        match event {
            crossterm::event::Event::Key(key_event) => {
                log::info!("handle key event {:?}", key_event);
                let focus: &mut Window = match self.tabs.index {
                    0 => &mut self.help_window,
                    1 => &mut self.file_manager_window,
                    2 => &mut self.search_window,
                    3 => &mut *self.text_window,
                    4 => &mut self.terminal_window,
                    5 => &mut self.color_window,
                    _ => panic!("app tabs index invalid"),
                };
                match focus.handle_event(event) {
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
                            KEY_F6 => self.on_select_color_tab(),
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
        let tw = &self.text_window;
        let tb = tw.text_buffer.lock().unwrap();
        self.debug_window.pen_col = tb.pen_col;
        self.debug_window.pen_row = tb.pen_col;
        self.debug_window.scroll_left = tw.scroll_left;
        self.debug_window.scroll_top = tw.scroll_top;
        EscalationEvent::Handled
    }

    fn reshape(&mut self, rect: &tui::layout::Rect) {
        log::info!("reshape({:?})", rect);
        self.bounds = *rect;
        // Main screen areas (aka "chunks").
        let chunks = Layout::default()
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(0),
                    Constraint::Length(5),
                    Constraint::Length(14),
                ]
                .as_ref(),
            )
            .split(self.bounds);
        self.tabs_window.reshape(&chunks[0]);
        for child in &mut self.child {
            child.reshape(&chunks[1]);
        }
        self.color_window.reshape(&chunks[1]);
        self.file_manager_window.reshape(&chunks[1]);
        self.help_window.reshape(&chunks[1]);
        self.search_window.reshape(&chunks[1]);
        self.terminal_window.reshape(&chunks[1]);
        self.text_window.reshape(&chunks[1]);
        self.debug_window.reshape(&chunks[2]);
        self.log_window.reshape(&chunks[3]);
    }
}
