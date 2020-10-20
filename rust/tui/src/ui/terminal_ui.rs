use crate::app::App;
use crate::ui;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Axis, Block, Borders, Paragraph, Row, Table, Tabs, Wrap},
    Frame,
};

pub fn draw_terminal_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Length(2), Constraint::Min(10)].as_ref())
        .split(area);
    draw_output(f, app, chunks[0]);
    draw_input(f, app, chunks[1]);
}

fn draw_output<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let tw = &app.text_window;
    let tb = tw.text_buffer.lock().unwrap();
    let text = tb
        .rows()
        .skip(tw.scroll_top)
        .take(10)
        .map(|s| Spans::from(s))
        .collect::<Vec<Spans>>();
    let block = Block::default().borders(Borders::NONE).title(Span::styled(
        "Output",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: false });
    f.render_widget(paragraph, area);
}

fn draw_input<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let tb = app.text_window.text_buffer.lock().unwrap();
    let text = tb
        .rows()
        .skip(app.text_window.scroll_top)
        .take(10)
        .map(|s| Spans::from(s))
        .collect::<Vec<Spans>>();
    let block = Block::default().borders(Borders::NONE).title(Span::styled(
        "Input",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(text).block(block); //.wrap(Wrap { trim: false });
    f.render_widget(paragraph, area);
}
