#![deny(unreachable_patterns)]

mod app;
mod buffer;
mod ui;
mod util;
mod proc;
use crate::app::App;
//use crate::proc;
//use crate::ui;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

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

    let foo = 43;
    let bar = match foo {
        22 => 3,
        _ => 0
    };

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
                //let current_event = event::read().unwrap();
                //if let CEvent::Key(key) = current_event {
                //    tx.send(Event::Input(key)).unwrap();
                //}
            }
        }
    });

    terminal.clear()?;

    const CTRL_DOWN : event::KeyEvent = event::KeyEvent {
        code: KeyCode::Down, modifiers: event::KeyModifiers::CONTROL};
    const CTRL_O : event::KeyEvent = event::KeyEvent {
        code: KeyCode::Char('o'), modifiers: event::KeyModifiers::CONTROL};
    const CTRL_Q : event::KeyEvent = event::KeyEvent {
        code: KeyCode::Char('q'), modifiers: event::KeyModifiers::CONTROL};
    const CTRL_UP : event::KeyEvent = event::KeyEvent {
        code: KeyCode::Up, modifiers: event::KeyModifiers::CONTROL};
    const KEY_DOWN : event::KeyEvent = event::KeyEvent {
        code: KeyCode::Down, modifiers: event::KeyModifiers::NONE};
    const KEY_F2 : event::KeyEvent = event::KeyEvent {
        code: KeyCode::F(2), modifiers: event::KeyModifiers::NONE};
    const KEY_F3 : event::KeyEvent = event::KeyEvent {
        code: KeyCode::F(3), modifiers: event::KeyModifiers::NONE};
    const KEY_LEFT : event::KeyEvent = event::KeyEvent {
        code: KeyCode::Left, modifiers: event::KeyModifiers::NONE};
    const KEY_PAGE_DOWN : event::KeyEvent = event::KeyEvent {
        code: KeyCode::PageDown, modifiers: event::KeyModifiers::NONE};
    const KEY_PAGE_UP : event::KeyEvent = event::KeyEvent {
        code: KeyCode::PageUp, modifiers: event::KeyModifiers::NONE};
    const KEY_RIGHT : event::KeyEvent = event::KeyEvent {
        code: KeyCode::Right, modifiers: event::KeyModifiers::NONE};
    const KEY_UP : event::KeyEvent = event::KeyEvent {
        code: KeyCode::Up, modifiers: event::KeyModifiers::NONE};

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
            },
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
