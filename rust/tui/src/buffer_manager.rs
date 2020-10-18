/// The BufferManager has a reference to every open buffer.
use crate::text_buffer::TextBuffer;
use std::fs::{File};
use std::io::prelude::*;
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
        let tb = Arc::new(Mutex::new(TextBuffer::new(
            std::path::Path::new(""),
            "<none>".to_string(),
        )));
        self.buffers.push(tb.clone());
        tb
    }

    pub fn load(&mut self, path: &std::path::Path) -> std::io::Result<Arc<Mutex<TextBuffer>>> {
        let data = std::fs::read_to_string(path)?;
        let tb = Arc::new(Mutex::new(TextBuffer::new(path, data)));
        self.buffers.push(tb.clone());
        Ok(tb)
    }

    pub fn save_all_files(&mut self) {
        for i in &self.buffers {
            let tb = i.lock().unwrap();
            let mut out_file = match File::create(&tb.file_path) {
                Err(e) => continue,
                Ok(f) => f,
            };
            out_file.write_all(tb.text_bytes()).unwrap();
            /*
            let mut stream = std::io::BufWriter::new(out_file).unwrap();
            for k in tb.text_iter() {
                stream.write(k);
            }
            stream.flush.unwrap();
            */
        }
    }
}
