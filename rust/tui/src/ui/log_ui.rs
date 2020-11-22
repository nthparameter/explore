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
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(area);
    let block = Block::default().borders(Borders::TOP).title(Span::styled(
        "Log",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let tw = &app.text_window;
    let tb = &tw.text_buffer.lock().unwrap();
    //let lines: &Vec<logging::Message> = &*logging::LOGGER.data.lock().unwrap().iter().map(
      //      |x| &x.message);
    //let lines: Vec<&str> = logging::LOGGER.data.lock().unwrap().iter().map(
    //        |x| x.message.as_str()).collect::<Vec<_>>();
    let lines: Vec<String> = logging::LOGGER.data.lock().unwrap().iter().map(
            |x| x.message.to_string()).collect::<Vec<_>>();
    let lines: Vec<Spans> = lines.iter().map(
            |x| Spans::from(x.as_str())).collect::<Vec<Spans>>();
    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);
}
