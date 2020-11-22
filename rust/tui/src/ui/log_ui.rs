use crate::app::App;
use crate::logging;
use crate::ui;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, Borders, Paragraph, Row,
        Wrap,
    },
    Frame,
};

pub fn draw_log_panel<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let block = Block::default().borders(Borders::TOP).title(Span::styled(
        "Log",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let lines: Vec<Spans> = logging::LOGGER.data.lock().unwrap().iter().rev()
    .take(area.height as usize - 1).map(
            |x| Spans::from(format!("[{}]{}:{}:{}",
                    x.level,
                    x.file.as_ref().unwrap_or(&"<none>".to_string()),
                    x.line.unwrap_or(0),
                    x.message.to_string()))).rev().collect::<Vec<_>>();
    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);
}
