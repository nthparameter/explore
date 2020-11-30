use lazy_static::lazy_static;
use std::fmt;
use std::sync::{Arc, Mutex};

/// Store information for a single logging message.
pub struct Message {
    pub file: Option<String>,
    pub level: log::Level,
    pub line: Option<u32>,
    pub message: String,
    pub module_path: Option<String>,
    pub target: String,
    pub thread: Option<String>,
    pub thread_id: std::thread::ThreadId,
}

/// Singleton for holding all log messages.
pub struct Logging {
    pub data: Arc<Mutex<Vec<Message>>>,
}

impl log::Log for Logging {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let message = Message {
                file: record.file().map(|x| x.to_string()),
                level: record.level(),
                line: record.line(),
                message: record.args().to_string(),
                module_path: record.module_path().map(|x| x.to_string()),
                target: record.target().to_string(),
                thread: std::thread::current().name().map(|x| x.to_string()),
                thread_id: std::thread::current().id(),
            };
            self.data.lock().unwrap().push(message);
        }
    }

    fn flush(&self) {}
}

lazy_static! {
    pub static ref LOGGER: Logging = Logging {
        data: Arc::new(Mutex::new(vec![])),
    };
}

/// Initialized the logging system. Call once.
pub fn init() -> Result<(), log::SetLoggerError> {
    log::set_logger(&*LOGGER).map(|()| log::set_max_level(log::LevelFilter::Info))
}
