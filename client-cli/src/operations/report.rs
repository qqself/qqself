use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    process::exit,
};

use qqself_core::db::{Record, DB};
use structopt::StructOpt;
use tracing::error;

#[derive(StructOpt, Debug)]
#[structopt(about = "Read the journal and report current state of things")]
pub struct ReportOpts {
    /// Path to journal file with all the entries
    #[structopt(short, long, default_value = "journal.txt")]
    journal_path: String,
}

#[tracing::instrument(level = "trace", skip_all)]
pub fn report(opts: ReportOpts) {
    let journal_path = Path::new(&opts.journal_path);
    if !journal_path.exists() {
        error!("Journal file does not exists at {:?}", journal_path);
        exit(1);
    }
    let file = File::open(journal_path).expect("Journal path should point to the openable file");
    let reader = BufReader::new(file);
    let mut db = DB::new();
    reader.lines().for_each(|line| {
        let line = line.expect("Cannot read journal line");
        if line.trim().starts_with('#') {
            return; // Skip the comments
        }
        if line.trim().is_empty() {
            return; // Skip empty lines
        }
        // Parse the record to see if it's a valid one
        let entry = qqself_core::parser::Parser::new(&line)
            .parse_date_record(None, None)
            .expect("Line should be a parsable record");
        db.add(Record::from_entry(entry, 0))
    });
    for skill in db.skills() {
        println!("{}", skill);
    }
}
