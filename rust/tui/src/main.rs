mod app;
mod ui;
mod util;
use crate::app::App;
//use crate::ui;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::{
    error::Error,
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


fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    // Set up input handling
    let (tx, rx) = mpsc::channel();

    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            // poll for tick rate duration, if no events, sent tick event.
            if event::poll(tick_rate - last_tick.elapsed()).unwrap() {
                if let CEvent::Key(key) = event::read().unwrap() {
                    tx.send(Event::Input(key)).unwrap();
                }
            }
            if last_tick.elapsed() >= tick_rate {
                tx.send(Event::Tick).unwrap();
                last_tick = Instant::now();
            }
        }
    });

    let mut app = App::new("Crossterm Demo", true);

    terminal.clear()?;

    const CTRL_Q : event::KeyEvent = event::KeyEvent {
        code: KeyCode::Char('q'), modifiers: event::KeyModifiers::CONTROL};
    const KEY_DOWN : event::KeyEvent = event::KeyEvent {
        code: KeyCode::Down, modifiers: event::KeyModifiers::NONE};
    const KEY_UP : event::KeyEvent = event::KeyEvent {
        code: KeyCode::Up, modifiers: event::KeyModifiers::NONE};

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;
        match rx.recv()? {
            Event::Input(event) => match event {
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
                KEY_DOWN => app.on_down(),
                KEY_UP => app.on_up(),
                _ => {}
            },
            Event::Tick => {
                app.on_tick();
            }
        }
            /*
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    let mods = event.modifiers;
                    if mods == event::KeyModifiers::CONTROL {
                    //if mods.contains(event::KeyModifiers::CONTROL) {
                        disable_raw_mode()?;
                        execute!(
                            terminal.backend_mut(),
                            LeaveAlternateScreen,
                            DisableMouseCapture
                        )?;
                        terminal.show_cursor()?;
                        break;
                    }
                }
                KeyCode::Char(c) => app.on_key(c),
                KeyCode::Left => app.on_left(),
                KeyCode::Up => app.on_up(),
                KeyCode::Right => app.on_right(),
                KeyCode::Down => app.on_down(),
                _ => {}
            },
            Event::Tick => {
                app.on_tick();
            }
        }
        */
        if app.should_quit {
            break;
        }
    }

    Ok(())
}
