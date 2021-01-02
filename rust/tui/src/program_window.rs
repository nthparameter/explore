/*
use crate::app::{App};
use crate::window::{EscalationEvent, Window};
use crate::text_window::TextWindow;


pub struct ProgramWindow {
    child: Vec<Box<Window>>,
}

impl ProgramWindow {
    pub fn new(app: &mut App) -> Self {
        let text_window =
            TextWindow::new(app.buffer_manager.new_text_buffer(std::path::Path::new(&"")));
        Self {
            child: vec![Box::new(text_window)],
        }
    }
/*
    pub fn focused_window() -> &Window {
    }*/
}

impl<'a> Window for ProgramWindow {
    fn handle_event(&mut self, event: &crossterm::event::Event) -> EscalationEvent {
        log::info!("app handle_event()");
        if let crossterm::event::Event::Key(key_event) = event {
            match self.child.last().expect("").handle_event(event) {
                _ => return EscalationEvent::Unhandled,
            }
            match *key_event {
                /*CTRL_N => self.new_file(),
                CTRL_O => self.on_select_open_tab(),
                CTRL_S => self.save_all_files(),
                CTRL_W => self.close_file(),
                CTRL_Q => self.should_quit = true,
                KEY_F1 => self.on_select_help_tab(),
                KEY_F2 => self.on_select_open_tab(),
                KEY_F3 => self.on_select_search_tab(),
                KEY_F4 => self.on_select_editor_tab(),
                KEY_F5 => self.on_select_terminal_tab(),*/
                _ => return EscalationEvent::Unhandled,
            }
        }
        EscalationEvent::Handled
    }
}
*/
