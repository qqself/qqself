extern crate chrono;
extern crate core;

use std::io;
use std::io::Write;
use std::process::exit;

use storage::Storage;

mod storage;
mod time;

fn main() -> io::Result<()> {
    let storage = Storage::new("/tmp/file".to_string());
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        match core::parser::Entry::from_string(&line) {
            Ok(entry) => storage.append(entry),
            Err(err) => {
                println!("ERROR {:#?}", err);
            }
        }
    }
}
