use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    process::exit,
    str::FromStr,
};

use qqself_core::{
    date_time::datetime::DateDay,
    db::{Record, DB},
};
use structopt::{clap::arg_enum, StructOpt};
use tracing::error;

arg_enum! {
    #[derive(Debug)]
    enum TimePeriod {
        Day,
        Week,
        Month,
        Year,
    }
}

#[derive(StructOpt, Debug)]
#[structopt(about = "Read the journal and report current state of things")]
pub struct ReportOpts {
    /// Path to journal file with all the entries
    #[structopt(short, long, default_value = "journal.txt")]
    journal_path: String,

    /// Period of time to make a report for
    #[structopt(short, long, possible_values = &TimePeriod::variants(), case_insensitive = true, default_value = "day")]
    period: TimePeriod,
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
            .parse_date_record()
            .unwrap_or_else(|err| panic!("error parsing the line '{line}' - {err}"));
        db.add(Record::from_entry(entry, 0))
    });
    println!("Skills:");
    for skill in db.skills() {
        println!("{}", skill);
    }

    let (start, end) = journal_range(opts.period);
    println!("Journal for range: {} - {}", start, end);

    for (day, journal_day) in db.journal().range(start..end) {
        println!("Day {}", day);
        for entry in &journal_day.entries {
            println!("\t{}", entry.to_string_short());
        }
    }
}

fn journal_range(period: TimePeriod) -> (DateDay, DateDay) {
    let now = chrono::Local::now();
    let end = DateDay::from_str(&now.date_naive().to_string()).unwrap();
    let start = end.remove_days(match period {
        TimePeriod::Day => 0,
        TimePeriod::Week => 7,
        TimePeriod::Month => 30,
        TimePeriod::Year => 365,
    });
    (start, end)
}
