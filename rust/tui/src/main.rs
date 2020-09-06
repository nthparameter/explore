#![deny(unreachable_patterns)]

mod app;
mod buffer;
mod key_const;
mod proc;
mod ui;
mod util;
use crate::app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use key_const::*;

use std::{
    error::Error as ErrorTrait,
    io::{stdout, Write},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
use tui::{backend::CrosstermBackend, Terminal};

enum Event<I> {
    Input(I),
    Tick,
}

fn main() -> Result<(), Box<dyn ErrorTrait>> {
    //proc::test_subprocesses()?;
    //futures::executor::block_on(proc::test_async_subprocesses());
    start_tui()
}

fn start_tui() -> Result<(), Box<dyn ErrorTrait>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    // Set up input handling
    let (tx, rx) = mpsc::channel();

    let mut app = App::new("Crossterm Demo", true);

    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            // poll for tick rate duration, if no events, sent tick event.
            let elapsed = last_tick.elapsed();
            if elapsed >= tick_rate {
                tx.send(Event::Tick).unwrap();
                last_tick = Instant::now();
            } else if event::poll(tick_rate - elapsed).unwrap() {
                tx.send(Event::Input(event::read().unwrap()));
            }
        }
    });

    terminal.clear()?;

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;
        match rx.recv()? {
            Event::Input(event) => {
                app.debug_event = event;
                if let crossterm::event::Event::Key(key_event) = event {
                    match key_event {
                        CTRL_DOWN => app.on_scroll_down(),
                        CTRL_O => app.on_open_file(),
                        CTRL_Q => {
                            disable_raw_mode()?;
                            execute!(
                                terminal.backend_mut(),
                                LeaveAlternateScreen,
                                DisableMouseCapture
                            )?;
                            terminal.show_cursor()?;
                            break;
                        }
                        CTRL_UP => app.on_scroll_up(),
                        KEY_DOWN => app.on_cursor_down(),
                        KEY_LEFT => app.on_cursor_left(),
                        KEY_F2 => app.on_select_editor_tab(),
                        KEY_F3 => app.on_select_terminal_tab(),
                        KEY_PAGE_DOWN => app.on_page_down(),
                        KEY_PAGE_UP => app.on_page_up(),
                        KEY_RIGHT => app.on_cursor_right(),
                        KEY_UP => app.on_cursor_up(),
                        _ => {}
                    }
                }
            }
            Event::Tick => {
                app.on_tick();
            }
        }
        if app.should_quit {
            break;
        }
    }

    Ok(())
}
