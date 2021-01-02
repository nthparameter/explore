
#[derive(PartialEq)]
pub enum EscalationEvent {
    Unhandled,
    Handled,
    //CloseMe,
    QuitProgram,
    /*Save,
    SaveAll,*/
}

/*enum WindowEvent {
    /// An event passed down from a parent window.
    Delegation(crossterm::event::Event),
    /// An event bubbled up from a child to a parent.
    Escalation(EscalationEvent),
}*/

/// A 'window' is a rectangular area of the screen that is capable of:
/// - being focused/unfocused
/// - accepting events to (potentially) handle
/// - changing proportions
pub trait Window {
    /// Whether the x,y position lies within the bounds of the window.
    fn contains(&self, x: u16, y: u16) -> bool;

    /// Returns true if the event was handled.
    fn handle_event(&mut self, event: &crossterm::event::Event) -> EscalationEvent;

    /// Change the bounds of window.
    fn reshape(&mut self, rect: &tui::layout::Rect);

}
