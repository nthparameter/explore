//! Traits and events for window handling. A window in this is any rectangular
//! area that can handle events.

use crate::app::{App, AppFrame};
use tui::backend::Backend;

#[derive(PartialEq)]
pub enum EscalationEvent {
    Unhandled,
    Handled,
    //CloseWindow,
    QuitProgram,
    /*Save,
    SaveAll,*/
}

/// A 'window' is a rectangular area of the screen that is capable of:
/// - being focused/unfocused
/// - accepting events to (potentially) handle
/// - changing proportions
pub trait Window {
    /// Whether the x,y position lies within the bounds of the window.
    fn contains(&self, x: u16, y: u16) -> bool;

    /// Draw window into frame.
    fn draw(&self, frame: &mut AppFrame);

    /// Returns true if the event was handled.
    fn handle_event(&mut self, event: &crossterm::event::Event) -> EscalationEvent;

    /// Change the bounds of window.
    fn reshape(&mut self, rect: &tui::layout::Rect);
}
