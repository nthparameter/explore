//! User interface for the application. This is the main (or root) UI element
//! that contains (holds as children) all the other UI elements.

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

/// Root draw call for the application. This will call draw() on all needed
/// nested UI elements.
pub fn draw<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
    // app.area_handler.clear();
    // Main screen areas (aka "chunks").
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(5),
                Constraint::Length(14),
            ]
            .as_ref(),
        )
        .split(frame.size());
    // Create UI tabs (labels for tabs).
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
    frame.render_widget(tabs, chunks[0]);
    // Draw tab contents (body of tab).
    match app.tabs.index {
        0 => help_ui::draw_help_tab(frame, app, chunks[1]),
        1 => file_manager_ui::draw_file_manager_tab(frame, app, chunks[1]),
        2 => search_ui::draw_search_tab(frame, app, chunks[1]),
        3 => text_editor_ui::draw_text_editor_tab(frame, app, chunks[1]),
        4 => terminal_ui::draw_terminal_tab(frame, app, chunks[1]),
        _ => {}
    };
    // Draw debug view.
    debug_ui::draw_debug_panel(frame, app, chunks[2]);
    log_ui::draw_log_panel(frame, app, chunks[3]);
    // app.area_handler.append(Area::new("app", frame.size()));
}
