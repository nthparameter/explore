//! User interface to display help to the user.

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

pub fn draw_help_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let block = Block::default().borders(Borders::NONE).title(Span::styled(
        "Help",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let text = vec![Spans::from(format!("{}", "This will be helpful someday."))];
    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}
