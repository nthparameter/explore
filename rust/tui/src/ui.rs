use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle},
    widgets::{
        Axis, BarChart, Block, Borders, Chart, Dataset, Gauge, List, ListItem, Paragraph, Row,
        Sparkline, Table, Tabs, Wrap,
    },
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // Main screen areas.
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)].as_ref())
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
        0 => draw_text_editor_tab(f, app, chunks[1]),
        1 => draw_second_tab(f, app, chunks[1]),
        _ => {}
    };
    // Draw debug view.
    draw_debug_panel(f, app, chunks[2]);
}

fn draw_text_editor_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(area);
    draw_text(f, app, chunks[1]);

    let block = Block::default().borders(Borders::NONE).title(Span::styled(
        "Animals",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let text = vec![Spans::from(format!("{}", app.progress))];
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[0]);
    f.set_cursor(app.pen_col as u16, app.pen_row as u16);
}

fn draw_text<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let text = app
        .buffer.text.lines().skip(app.scroll_top).take(10)
        .map(|s| {Spans::from(s)}).collect::<Vec<Spans>>();
    let block = Block::default().borders(Borders::TOP).title(Span::styled(
        &app.buffer.name,
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
    /*
    let text = vec![
        Spans::from("This is a paragraph with several lines. You can change style your text the way you want"),
        Spans::from("This is a paragraph with several lines. You can change style your text the way you want"),
        Spans::from("This is a paragraph with several lines. You can change style your text the way you want"),
        Spans::from(""),
        Spans::from(vec![
            Span::from("For example: "),
            Span::styled("under", Style::default().fg(Color::Red)),
            Span::raw(" "),
            Span::styled("the", Style::default().fg(Color::Green)),
            Span::raw(" "),
            Span::styled("rainbow", Style::default().fg(Color::Blue)),
            Span::raw("."),
        ]),
        Spans::from(vec![
        Spans::from(
            "One more thing is that it should display unicode characters: 10€"
        ),
    ];
    let block = Block::default().borders(Borders::NONE).title(Span::styled(
        "Log",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
    */
}

fn draw_output<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let text = app.buffer
        .text.lines()
        .skip(app.scroll_top)
        .take(10)
        .map(|s| {Spans::from(s)})
        .collect::<Vec<Spans>>();
    let block = Block::default().borders(Borders::NONE).title(Span::styled(
        "Output",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

fn draw_input<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let text = app.buffer.text.lines().skip(app.scroll_top).take(10)
       .map(|s| {Spans::from(s)}).collect::<Vec<Spans>>();
    let block = Block::default().borders(Borders::NONE).title(Span::styled(
        "Input",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

fn draw_second_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Length(2), Constraint::Min(10)].as_ref())
        .split(area);
    draw_output(f, app, chunks[0]);
    draw_input(f, app, chunks[1]);
}

fn draw_debug_panel<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(area);
    let text = vec![
        Spans::from(format!("pen r:{} c:{}", app.pen_row, app.pen_col)),
        Spans::from(format!("in:{:?}", app.debug_event)),
    ];
    let block = Block::default().borders(Borders::TOP).title(Span::styled(
        "Debug",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}
