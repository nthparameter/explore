
/// The BufferManager has a reference to every open buffer.

use crate::text_buffer::TextBuffer;
use std::sync::{Arc, Mutex};

pub struct BufferManager {
    buffers: Vec<Arc<Mutex<TextBuffer>>>,
}

impl BufferManager {
    pub fn new() -> Self {
        Self {
            buffers: std::vec![],
        }
    }

    pub fn new_text_buffer(&mut self) -> Arc<Mutex<TextBuffer>> {
        let tb = Arc::new(Mutex::new(TextBuffer::new(std::path::Path::new(""),
                "<none>".to_string())));
        self.buffers.push(tb.clone());
        tb
    }

    pub fn load(&mut self, path: &std::path::Path) ->
            std::io::Result<Arc<Mutex<TextBuffer>>> {
        let data = std::fs::read_to_string(path)?;
        let tb = Arc::new(Mutex::new(TextBuffer::new(path, data)));
        self.buffers.push(tb.clone());
        Ok(tb)
    }
}
