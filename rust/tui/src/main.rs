//! Main entry point. This is where the store begins.

#![deny(unreachable_patterns)]

mod app;
mod args;
mod buffer_manager;
mod debug_window;
mod key_const;
mod log_window;
mod logging;
//mod open_file_view;
mod proc;
mod program_window;
mod tabs_window;
mod text_buffer;
mod text_window;
mod ui;
mod util;
mod window;

use crate::app::App;
use crate::args::CmdArgs;
use crate::window::Window;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use key_const::*;
use log;
//use scopeguard;
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
    logging::init()?;
    log::info!("Enter main()");
    let cmd_args: CmdArgs = argh::from_env();
    if cmd_args.version {
        println!("Editor version 0.0.1");
        return Ok(());
    }
    start_tui(cmd_args)?;
    log::info!("Exit main()");
    Ok(())
}

/// RAII wrapper to undo changes to the terminal on exit.
struct FullMouseTerminal(Terminal<CrosstermBackend<std::io::Stdout>>);

impl FullMouseTerminal {
    fn new() -> Result<Self, crossterm::ErrorKind> {
        let mut stdout = stdout();
        enable_raw_mode()?;
        if let Err(e) = execute!(stdout, EnterAlternateScreen, EnableMouseCapture) {
            disable_raw_mode().expect("disable raw terminal mode.");
            return Err(e);
        }
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;
        Ok(Self(terminal))
    }
}

impl Drop for FullMouseTerminal {
    fn drop(&mut self) {
        execute!(
            self.0.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .expect("");
        disable_raw_mode().expect("disable raw terminal mode.");
        self.0.show_cursor().expect("show terminal cursor.");
    }
}

/// A helper function for 'main()' so that main is a little cleaner.
fn start_tui(cmd_args: CmdArgs) -> Result<(), Box<dyn ErrorTrait>> {
    let mut terminal = FullMouseTerminal::new()?;

    // Set up input handling.
    let (tx, rx) = mpsc::channel();

    let mut app = App::new("Editor");

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
                tx.send(Event::Input(event::read().unwrap()))
                    .expect("Send event on channel.");
            }
        }
    });

    for i in cmd_args.file_paths {
        app.open_file(std::path::Path::new(&i));
    }

    // Handle events until `app.should_quit`.
    while !app.should_quit {
        let mut start_draw = Instant::now();
        //terminal.0.draw(|frame| ui::app_ui::draw(frame, &mut app))?;
        terminal.0.draw(|frame| app.draw(frame))?;
        app.debug_window.draw_time = start_draw.elapsed();
        app.draw_time = app.debug_window.draw_time;

        match rx.recv()? {
            Event::Input(event) => {
                app.handle_event(&event);
            }
            Event::Tick => {
                app.on_tick();
            }
        }
    }

    Ok(())
}
