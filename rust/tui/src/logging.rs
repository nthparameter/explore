
use log::{Record, Level, Metadata};
use log::{SetLoggerError, LevelFilter};
use std::fmt;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

pub struct Message {
    pub message: String,
}

pub struct Logging {
    pub data: Arc<Mutex<Vec<Message>>>,
}

impl log::Log for Logging {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let message = Message{
                message: record.args().to_string(),
            };
            self.data.lock().unwrap().push(message);
            //println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

lazy_static! {
    pub static ref LOGGER: Logging = Logging {
        data: Arc::new(Mutex::new(vec![])),
    };
}

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&*LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
}
