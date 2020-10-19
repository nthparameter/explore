use crate::app::App;
//use crate::ui;
use crate::ui::debug_ui;
use crate::ui::terminal_ui;
use crate::ui::text_editor_ui;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Axis, BarChart, Block, Borders, Chart, Dataset, Gauge, List, ListItem, Paragraph, Row,
        Sparkline, Table, Tabs, Wrap,
    },
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // Main screen areas.
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(4),
            ]
            .as_ref(),
        )
        .split(f.size());
    // Create tabs (labels for tabs).
    let tab_titles = app
        .tabs
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect();
    let tabs = Tabs::new(tab_titles)
        .block(Block::default().borders(Borders::BOTTOM).title(app.title))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.index);
    f.render_widget(tabs, chunks[0]);
    // Draw tab contents (body of tab).
    match app.tabs.index {
        0 => text_editor_ui::draw_text_editor_tab(f, app, chunks[1]),
        1 => terminal_ui::draw_second_tab(f, app, chunks[1]),
        _ => {}
    };
    // Draw debug view.
    debug_ui::draw_debug_panel(f, app, chunks[2]);
}
