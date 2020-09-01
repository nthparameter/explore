mod app;
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

//use std::process::{Command, Stdio};
//use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn main() -> Result<(), Box<dyn ErrorTrait>> {
    //proc::test_subprocesses();
    futures::executor::block_on(proc::test_async_subprocesses());

    /*
    println!("Hello, world!");
    let (tx, rx)  = std::sync::mpsc::channel();
    let tx2 = tx.clone();
    {
    let stdout = Command::new("repeat.exe")
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other, "bytes"))?;
    std::thread::spawn(move || {
        stdout.bytes()
            .filter_map(|b| b.ok())
            .for_each(|b| {tx.send(String::from_utf8(vec![b]).unwrap());});
    });
    }
    println!("Hello, world!");
    {
    let stdout = Command::new("repeat.exe")
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other, "lines"))?;
    let reader = BufReader::new(stdout);
    std::thread::spawn(move || {
    reader
        .lines()
        .filter_map(|line| line.ok())
        //.filter(|line| line.find("<").is_some())
        //.for_each(|line| println!("{}", line));
        .for_each(|line| {tx2.send(line);});
    });
    }
    let mut count = 0;
    while count < 40 {
        println!("{:?}", rx.recv().unwrap());
        std::thread::sleep(std::time::Duration::from_millis(250));
        count += 1;
    }
    */
    Ok(())
}

fn start_tui() -> Result<(), Box<dyn ErrorTrait>> {
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
            let elapsed = last_tick.elapsed();
            if elapsed >= tick_rate {
                tx.send(Event::Tick).unwrap();
                last_tick = Instant::now();
            } else if event::poll(tick_rate - elapsed).unwrap() {
                if let CEvent::Key(key) = event::read().unwrap() {
                    tx.send(Event::Input(key)).unwrap();
                }
            }
        }
    });

    let mut app = App::new("Crossterm Demo", true);

    terminal.clear()?;

    const CTRL_O : event::KeyEvent = event::KeyEvent {
        code: KeyCode::Char('o'), modifiers: event::KeyModifiers::CONTROL};
    const CTRL_Q : event::KeyEvent = event::KeyEvent {
        code: KeyCode::Char('q'), modifiers: event::KeyModifiers::CONTROL};
    const KEY_DOWN : event::KeyEvent = event::KeyEvent {
        code: KeyCode::Down, modifiers: event::KeyModifiers::NONE};
    const KEY_F2 : event::KeyEvent = event::KeyEvent {
        code: KeyCode::F(2), modifiers: event::KeyModifiers::NONE};
    const KEY_F3 : event::KeyEvent = event::KeyEvent {
        code: KeyCode::F(3), modifiers: event::KeyModifiers::NONE};
    const KEY_PAGE_DOWN : event::KeyEvent = event::KeyEvent {
        code: KeyCode::PageDown, modifiers: event::KeyModifiers::NONE};
    const KEY_PAGE_UP : event::KeyEvent = event::KeyEvent {
        code: KeyCode::PageUp, modifiers: event::KeyModifiers::NONE};
    const KEY_UP : event::KeyEvent = event::KeyEvent {
        code: KeyCode::Up, modifiers: event::KeyModifiers::NONE};

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;
        match rx.recv()? {
            Event::Input(event) => match event {
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
                KEY_DOWN => app.on_down(),
                KEY_F2 => app.on_select_editor_tab(),
                KEY_F3 => app.on_select_terminal_tab(),
                KEY_PAGE_DOWN => app.on_page_down(),
                KEY_PAGE_UP => app.on_page_up(),
                KEY_UP => app.on_up(),
                _ => {}
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
