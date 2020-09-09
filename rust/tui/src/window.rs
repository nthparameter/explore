



pub trait EventHandler {
    fn handle_event(&mut self, event: &crossterm::event::Event);
}