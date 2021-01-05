use crate::app::App;
use crate::ui;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Row, Tabs, Wrap},
    Frame,
};

pub fn draw_debug_panel<B>(frame: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(area);
    let block = Block::default().borders(Borders::TOP).title(Span::styled(
        "Debug",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let tw = &app.text_window;
    let tb = &tw.text_buffer.lock().unwrap();
    let text = vec![
        Spans::from(format!("scroll t:{} l:{}", tw.scroll_top, tw.scroll_left)),
        Spans::from(format!("pen r:{} c:{}", tb.pen_row, tb.pen_col)),
        Spans::from(format!("in:{:?}", app.debug_event)),
        Spans::from(format!(" draw:{:>8}", format!("{:.3?}", app.draw_time))),
    ];
    let paragraph = Paragraph::new(text).block(block); //.wrap(Wrap { trim: false });
    frame.render_widget(paragraph, area);
}
