use crate::app::App;
use crate::ui::debug_ui;
use crate::ui::file_manager_ui;
use crate::ui::help_ui;
use crate::ui::log_ui;
use crate::ui::search_ui;
use crate::ui::terminal_ui;
use crate::ui::text_editor_ui;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Axis, BarChart, Block, Borders, Paragraph, Row, Tabs, Wrap},
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
        0 => help_ui::draw_help_tab(f, app, chunks[1]),
        1 => file_manager_ui::draw_file_manager_tab(f, app, chunks[1]),
        2 => text_editor_ui::draw_text_editor_tab(f, app, chunks[1]),
        3 => search_ui::draw_search_tab(f, app, chunks[1]),
        4 => terminal_ui::draw_terminal_tab(f, app, chunks[1]),
        _ => {}
    };
    // Draw debug view.
    debug_ui::draw_debug_panel(f, app, chunks[2]);
    log_ui::draw_log_panel(f, app, chunks[3]);
}
