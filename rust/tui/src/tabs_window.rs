//! User interface window.

use crate::app::{App, AppFrame};
use crate::window::{EscalationEvent, Window};
use log;
use tui::layout::{Constraint, Direction, Layout, Rect};

/// A collection of tabs to select different views.
#[derive(Default)]
pub struct TabsWindow {
    child: Vec<Box<Window>>,
    bounds: tui::layout::Rect,
}

impl Window for TabsWindow {
    fn contains(&self, x: u16, y: u16) -> bool {
        tui::layout::Rect::new(x, y, 0, 0).intersects(self.bounds)
    }

    fn draw(&self, frame: &mut AppFrame) {}

    fn handle_event(&mut self, event: &crossterm::event::Event) -> EscalationEvent {
        log::info!("handle_event() {:?}", event);
        EscalationEvent::Unhandled
    }

    fn reshape(&mut self, rect: &tui::layout::Rect) {
        log::info!("reshape({:?})", rect);
        self.bounds = *rect;
    }
}
