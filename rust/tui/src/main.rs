#![deny(unreachable_patterns)]

mod app;
mod args;
mod buffer_manager;
mod key_const;
//mod open_file_view;
mod proc;
mod text_buffer;
mod text_window;
mod ui;
mod util;
mod window;

use crate::app::App;
use crate::args::CmdArgs;
use crate::window::EventHandler;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
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
    let cmd_args: CmdArgs = argh::from_env();
    if cmd_args.version {
        println!("Editor version 0.0.1");
        return Ok(());
    }
    start_tui(cmd_args)
}

fn start_tui(cmd_args: CmdArgs) -> Result<(), Box<dyn ErrorTrait>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    // Set up input handling
    let (tx, rx) = mpsc::channel();

    let mut app = App::new("Editor", true);

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

    for i in cmd_args.file_paths {
        app.open_file(std::path::Path::new(&i));
    }

    // Handle events until `app.should_quit`.
    loop {
        terminal.draw(|f| ui::app_ui::draw(f, &mut app))?;
        match rx.recv()? {
            Event::Input(event) => {
                app.debug_event = event;
                app.handle_event(&event);
                if app.should_quit {
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
