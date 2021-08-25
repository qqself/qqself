use core::parser::Entry;
use std::fs::OpenOptions;
use std::io::prelude::*;
use time::time_now;

pub struct Storage {
    path: String,
}

impl Storage {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn append(&self, entry: Entry) {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&self.path)
            .unwrap();
        file.write(entry.to_string().as_bytes()).unwrap();
    }
}
