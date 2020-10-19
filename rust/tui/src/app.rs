use crate::buffer_manager::BufferManager;
use crate::key_const::*;
use crate::text_window::TextWindow;
use crate::util::TabsState;
use crate::window::EventHandler;

pub struct App<'a> {
    pub buffer_manager: BufferManager,
    pub title: &'a str,
    pub tabs: TabsState<'a>,
    pub progress: f64,
    pub enhanced_graphics: bool,
    pub debug_event: crossterm::event::Event,
    pub should_quit: bool,
    pub text_window: TextWindow<'a>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool) -> App<'a> {
        let mut buffer_manager = BufferManager::new();
        let mut text_window = TextWindow::new(buffer_manager.new_text_buffer());

        App {
            buffer_manager,
            title,
            tabs: TabsState::new(vec!["Editor", "Terminal"]),
            progress: 0.0,
            enhanced_graphics,
            debug_event: crossterm::event::Event::Resize(1, 1),
            should_quit: false,
            text_window,
        }
    }

    pub fn close_file(&mut self) {
    }

    pub fn new_file(&mut self) {
    }

    pub fn on_open_file(&mut self) {
        let tb = self
            .buffer_manager
            .load(std::path::Path::new("src/app.rs"))
            .expect("read file");
        self.text_window.set_text_buffer(tb);
    }

    pub fn open_file(&mut self, file_path: &std::path::Path) {
        let tb = self
            .buffer_manager
            .load(std::path::Path::new(file_path))
            .expect("read file");
        self.text_window.set_text_buffer(tb);
    }

    pub fn save_all_files(&mut self) {
        self.buffer_manager.save_all_files();
    }

    pub fn on_select_editor_tab(&mut self) {
        self.tabs.index = 0;
    }

    pub fn on_select_terminal_tab(&mut self) {
        self.tabs.index = 1;
    }

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
                CTRL_N => self.new_file(),
                CTRL_O => self.on_open_file(),
                CTRL_S => self.save_all_files(),
                CTRL_W => self.close_file(),
                CTRL_Q => self.should_quit = true,
                KEY_F2 => self.on_select_editor_tab(),
                KEY_F3 => self.on_select_terminal_tab(),
                _ => self.text_window.handle_event(event),
            }
        }
    }
}
