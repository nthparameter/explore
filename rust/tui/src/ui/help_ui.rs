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
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(area);
    let h_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(7), Constraint::Min(0)].as_ref())
        .split(chunks[1]);
    draw_text(f, app, h_chunks[1]);

    let block = Block::default().borders(Borders::NONE).title(Span::styled(
        "Help",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let text = vec![Spans::from(format!("{}", app.progress))];
    let paragraph = Paragraph::new(text).block(block); //.wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[0]);
    //f.set_cursor(app.pen_col as u16, app.pen_row as u16);
}

fn draw_text<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let tw = &mut app.text_window;
    let tb = &tw.text_buffer.lock().unwrap();
    let block = Block::default().borders(Borders::TOP).title(Span::styled(
        &tb.name,
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let inner_area = block.inner(area);
    tw.render_width = inner_area.width as usize;
    tw.render_height = inner_area.height as usize;
    let text = tb
        .rows()
        .skip(tw.scroll_top)
        .take(area.height as usize)
        .map(|s| Spans::from(s))
        .collect::<Vec<Spans>>();
    let paragraph = Paragraph::new(text).block(block); //.wrap(Wrap { trim: false });
    f.render_widget(paragraph, area);

    // Show the cursor if it's in the view.
    let height = inner_area.height as usize;
    let width = inner_area.width as usize;
    if tw.scroll_top <= tb.pen_row
        && tw.scroll_top + height > tb.pen_row
        && tw.scroll_left <= tb.pen_col
        && tw.scroll_left + width > tb.pen_col
    {
        f.set_cursor(
            inner_area.x + (tb.pen_col - tw.scroll_left) as u16,
            inner_area.y + (tb.pen_row - tw.scroll_top) as u16,
        );
    }
}
