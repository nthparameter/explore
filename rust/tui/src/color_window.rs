//! User interface window.

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

/// A display of debug information for the program itself.
pub struct ColorWindow {
    bounds: tui::layout::Rect,
    pub debug_event: crossterm::event::Event,
    pub draw_time: std::time::Duration,
    pub pen_col: usize,
    pub pen_row: usize,
    pub scroll_top: usize,
    pub scroll_left: usize,
    pub tick_count: usize,
}

impl Default for ColorWindow {
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

impl Window for ColorWindow {
    fn contains(&self, x: u16, y: u16) -> bool {
        tui::layout::Rect::new(x, y, 0, 0).intersects(self.bounds)
    }

    fn draw(&self, frame: &mut AppFrame) {
        let block = Block::default().borders(Borders::TOP).title(Span::styled(
            "Color",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ));
        let text = (0..40).map(|y|
        Spans::from((0..80).map(
                |x| Span::styled("F",
                        Style::default().bg(Color::White).fg(
                                Color::Rgb(
                                    (x * 3) as u8,
                                    (y*16) as u8,
                                    (x as u32*y%256) as u8,
                                    )
                                )
                )).collect::<Vec<_>>()),
        ).collect::<Vec<_>>();
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
