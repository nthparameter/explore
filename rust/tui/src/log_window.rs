//! User interface for displaying log data.

use crate::app::{App, AppFrame};
use crate::logging;
use crate::window::{EscalationEvent, Window};
use log;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Row, Tabs, Wrap},
    Frame,
};

/// A text window that displays log output.
#[derive(Default)]
pub struct LogWindow {
    child: Vec<Box<Window>>,
    bounds: tui::layout::Rect,
}

impl Window for LogWindow {
    fn contains(&self, x: u16, y: u16) -> bool {
        tui::layout::Rect::new(x, y, 0, 0).intersects(self.bounds)
    }

    fn draw(&self, frame: &mut AppFrame) {
        let block = Block::default().borders(Borders::TOP).title(Span::styled(
            "Log",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ));
        if self.bounds.height < 1 {
            return;
        }
        let lines: Vec<Spans> = logging::LOGGER
            .data
            .lock()
            .unwrap()
            .iter()
            .rev()
            .take(self.bounds.height as usize - 1)
            .map(|x| {
                Spans::from(format!(
                    "[{}]{}:{}:{}",
                    x.level,
                    x.file.as_ref().unwrap_or(&"<none>".to_string()),
                    x.line.unwrap_or(0),
                    x.message.to_string()
                ))
            })
            .rev()
            .collect::<Vec<_>>();
        let paragraph = Paragraph::new(lines).block(block);
        frame.render_widget(paragraph, self.bounds);
    }

    fn handle_event(&mut self, event: &crossterm::event::Event) -> EscalationEvent {
        log::info!("handle_event() {:?}", event);
        EscalationEvent::Unhandled
    }

    fn reshape(&mut self, rect: &tui::layout::Rect) {
        log::info!("reshape({:?})", rect);
        self.bounds = *rect;
    }
}
