//! User interface to search across files and directories.

use crate::app::{App, AppFrame};
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

/// Interface for searching files and directories.
pub struct SearchWindow {
    bounds: tui::layout::Rect,
    pub debug_event: crossterm::event::Event,
    pub draw_time: std::time::Duration,
    pub pen_col: usize,
    pub pen_row: usize,
    pub scroll_top: usize,
    pub scroll_left: usize,
    pub tick_count: usize,
}

impl Default for SearchWindow {
    fn default() -> Self {
        Self {
            bounds: tui::layout::Rect::default(),
            debug_event: crossterm::event::Event::Resize(1, 1),
            draw_time: std::time::Duration::default(),
            pen_col: usize::default(),
            pen_row: usize::default(),
            scroll_top: usize::default(),
            scroll_left: usize::default(),
            tick_count: usize::default(),
        }
    }
}

impl Window for SearchWindow {
    fn contains(&self, x: u16, y: u16) -> bool {
        tui::layout::Rect::new(x, y, 0, 0).intersects(self.bounds)
    }

    fn draw(&self, frame: &mut AppFrame) {
        let block = Block::default().borders(Borders::TOP).title(Span::styled(
            "Search",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ));
        let text = vec![
            Spans::from(format!(
                "scroll t:{} l:{}",
                self.scroll_top, self.scroll_left
            )),
            Spans::from(format!("pen r:{} c:{}", self.pen_row, self.pen_col)),
            Spans::from(format!("in:{:?}", self.debug_event)),
            Spans::from(format!(
                "tick:{:2?} draw:{:>8}",
                self.tick_count,
                format!("{:.3?}", self.draw_time)
            )),
        ];
        let paragraph = Paragraph::new(text).block(block);
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
