//! User interface to search across files and directories.

use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Axis, BarChart, Block, Borders, Chart, Dataset, Gauge, List, ListItem, Paragraph, Row,
        Tabs, Wrap,
    },
    Frame,
};

/// Display the contents of search tab.
pub fn draw_search_tab<B>(frame: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let block = Block::default().borders(Borders::NONE).title(Span::styled(
        "Search in files",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let text = vec![Spans::from(format!("{}", "Work in progress."))];
    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);
}
