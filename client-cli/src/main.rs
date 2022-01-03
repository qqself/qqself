use chrono::prelude::*;
use std::borrow::Borrow;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use structopt::clap::arg_enum;
use structopt::StructOpt;

use crate::core::parser::Entry;
use core;

// Query data files
#[derive(StructOpt, Debug)]
struct Query {
    // Input folder with data
    #[structopt(short, long, parse(from_os_str))]
    input: PathBuf,

    // Time filter for the query. Only entries that fits into last specified period will be read
    #[structopt(long, possible_values = &TimeRange::variants(), case_insensitive = true, default_value = "All")]
    time: TimeRange,

    // Query filter - use the same syntax as for creating new entries and matched entries will
    // be returned
    #[structopt(name = "QUERY", multiple = true)]
    query: Vec<String>,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "development", version = "0.0.1")]
enum Opt {
    Query(Query),
}

arg_enum! {
    #[derive(Debug)]
    enum TimeRange {
        Day,
        Week,
        Month,
        Year,
        All,
    }
}

fn main() {
    let opt: Opt = Opt::from_args();
    match opt {
        Opt::Query(q) => query(q),
    }
}

fn query(opts: Query) {
    let min_date = date_filter(opts.time, chrono::Local::today());
    let mut results = Vec::new();
    for year in enumerate_folder_after(&opts.input, min_date.year() as u32) {
        let path = opts.input.join(&year);
        for month in enumerate_folder_after(&path, min_date.month()) {
            let path = path.join(&month);
            for day in enumerate_folder_after(&path, min_date.day()) {
                // TODO Remove hardcoded file extension
                let file = File::open(path.join(format!("{}.md", day))).unwrap();
                let mut prev = String::new();
                for line in BufReader::new(file).lines() {
                    let prefix = format!("{}-{}-{}", year, month, day);
                    if let Ok(entry) = Entry::from_string(&line.unwrap(), &prefix, &prev) {
                        prev = entry.date_range.to.clone();
                        println!("Entry {:?}", entry);
                    }
                }
            }
        }
    }

    // TODO Parse query - string join/parse
    // TODO Parse content and store only relevant to query
    // TODO Render results
}

fn enumerate_folder_after(path: &PathBuf, after: u32) -> Vec<String> {
    let mut entries: Vec<String> = std::fs::read_dir(path)
        .unwrap()
        .map(|v| {
            v.unwrap()
                .path()
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string()
        })
        .filter(|v| match v.parse::<u32>() {
            Ok(v) => v >= after,
            _ => false,
        })
        .collect();
    entries.sort();
    entries
}

fn date_filter(range: TimeRange, now: chrono::Date<Local>) -> Date<Local> {
    match range {
        TimeRange::Day => now,
        TimeRange::Week => {
            now - chrono::Duration::days(now.weekday().num_days_from_monday() as i64)
        }
        TimeRange::Month => chrono::Local.ymd(now.year(), now.month(), 1),
        TimeRange::Year => chrono::Local.ymd(now.year(), 1, 1),
        TimeRange::All => chrono::Local.ymd(1, 1, 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_filters() {
        let cases = vec![
            ((2021, 12, 31), TimeRange::Day, (2021, 12, 31)),
            ((2021, 12, 31), TimeRange::Week, (2021, 12, 27)),
            ((2021, 12, 31), TimeRange::Month, (2021, 12, 1)),
            ((2021, 12, 31), TimeRange::Year, (2021, 1, 1)),
            ((2021, 12, 31), TimeRange::All, (1, 1, 1)),
        ];
        for (now, range, want) in cases {
            let now = chrono::Local.ymd(now.0, now.1, now.2);
            let want = chrono::Local.ymd(want.0, want.1, want.2);
            let got = date_filter(range, now);
            assert_eq!(got, want);
        }
    }
}
