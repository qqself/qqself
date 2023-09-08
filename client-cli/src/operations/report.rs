use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    process::exit,
};

use clap::{Parser, ValueEnum};
use qqself_core::{
    date_time::datetime::DateDay,
    db::{Query, Record, DB},
};
use tracing::error;

#[derive(Debug, Clone, ValueEnum)]
enum TimePeriod {
    Day,
    Week,
    Month,
    Year,
}

#[derive(Parser, Debug)]
#[command(about = "Read the journal and report current state of things")]
pub struct ReportOpts {
    /// Path to journal file with all the entries
    #[arg(short, long, default_value = "journal.txt")]
    journal_path: String,

    /// Period of time to make a report for
    #[arg(short, long, value_enum, default_value = "day")]
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
        let record =
            Record::parse(&line).unwrap_or_else(|_| panic!("entry should be valid, line='{line}'"));
        db.add(record, false, None);
    });
    println!("Skills:");
    db.skills().iter().for_each(|(_, skill)| {
        println!("{}", skill);
    });

    let (start, end) = journal_range(opts.period);
    println!("Journal for range: {} - {}", start, end);

    let query = Query::new(&format!("filter after={} before={}", start, end))
        .expect("query should be valid");
    db.update_query(query);

    let mut prev_day = None;
    for entry in db.query_results().iter() {
        if !prev_day.is_some_and(|v| v == entry.date_range().start().date()) {
            prev_day.replace(entry.date_range().start().date());
            println!("Day {}", entry.date_range().start().date());
        }
        println!("\t{}", entry.to_string(true, false));
    }
}

fn journal_range(period: TimePeriod) -> (DateDay, DateDay) {
    let end = DateDay::today();
    let start = end.remove_days(match period {
        TimePeriod::Day => 0,
        TimePeriod::Week => 7,
        TimePeriod::Month => 30,
        TimePeriod::Year => 365,
    });
    (start, end)
}
