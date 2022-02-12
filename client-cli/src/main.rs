use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use chrono::{Datelike, Local, TimeZone};
use structopt::clap::arg_enum;
use structopt::StructOpt;

use core::datetime::{Date, DateTime, DateTimeRange, DayTime};
use core::db::{Query, DB};
use core::record::Record;

// Query data files
#[derive(StructOpt, Debug)]
struct QueryOpts {
    // Input folder with data
    #[structopt(short, long, parse(from_os_str))]
    input: PathBuf,

    // Time filter for the query. Only entries that fits into last specified period will be read
    #[structopt(long, possible_values = &TimeRange::variants(), case_insensitive = true, default_value = "All")]
    time: TimeRange,

    // Time filter minimum date in YYYY-MM-DD format
    #[structopt(long)]
    from: Option<Date>,

    // Time filter maximum date in YYYY-MM-DD format
    #[structopt(long)]
    to: Option<Date>,

    // Query filter - use the same syntax as for creating new entries and matched entries will
    // be returned
    #[structopt(name = "QUERY", multiple = true)]
    query: Vec<String>,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "development", version = "0.0.1")]
enum Opt {
    Query(QueryOpts),
}

arg_enum! {
    #[derive(Debug, PartialEq)]
    enum TimeRange {
        Day,
        Week,
        Month,
        Year,
        All,
    }
}

// TODO We assume that every day log has entries belonging to that day, but in
//      reality it may shift to the next one as entries could be after midnight
//      as well
fn main() {
    let opt: Opt = Opt::from_args();
    match opt {
        Opt::Query(q) => query(q),
    }
}

fn query(opts: QueryOpts) {
    let db = fill_db(&opts);
    if opts.query.is_empty() {
        // TODO Render goals
    } else {
        let query_str = opts.query.join(" ");
        let date_filter = date_filter(opts.time, opts.from, opts.to, chrono::Local::today());
        for stats in db
            .query(Query::new(&query_str, Some(date_filter)).unwrap())
            .unwrap()
        {
            for entry in &stats.entries {
                println!("{}", entry);
            }
            println!("----------");
            print!("Duration {}, Count {}", stats.duration(), stats.count());
            for (tag, total) in &stats.prop_totals() {
                print!(", {} {}", tag, total);
            }
            println!("\n")
        }
    }
}

fn fill_db(opts: &QueryOpts) -> DB {
    let mut db = DB::new();
    for year in enumerate_folder(&opts.input) {
        let path = opts.input.join(&year);
        for month in enumerate_folder(&path) {
            let path = path.join(&month);
            for day in enumerate_folder(&path) {
                let mut prev_record_end = None;
                let prefix = format!("{}-{}-{}", year, month, day);
                let date = prefix.parse::<Date>().unwrap();
                // TODO Remove hardcoded file extension
                let file = File::open(path.join(format!("{}.md", day))).unwrap();
                for line in BufReader::new(file).lines() {
                    if let Ok(record) =
                        Record::from_string(&line.unwrap(), date.clone(), prev_record_end.clone())
                    {
                        if let Record::Entry(entry) = &record {
                            prev_record_end = Some(entry.date_range.end.time.clone());
                        }
                        db.add(record);
                    }
                }
            }
        }
    }
    db
}

fn enumerate_folder(path: &Path) -> Vec<String> {
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
        .filter(|v| v.parse::<usize>().is_ok())
        .collect();
    entries.sort();
    entries
}

fn date_filter(
    range: TimeRange,
    from: Option<Date>,
    to: Option<Date>,
    now: chrono::Date<Local>,
) -> DateTimeRange {
    let date_from_chrono =
        |d: chrono::Date<Local>| Date::new(d.year() as u16, d.month() as u8, d.day() as u8);
    let mut out = DateTimeRange::new(
        DateTime::new(Date::new(1, 1, 1), DayTime::new(0, 0)),
        DateTime::new(Date::new(3000, 1, 1), DayTime::new(23, 59)),
    );
    if let Some(from) = from {
        out.start.date = from;
    }
    if let Some(to) = to {
        out.end.date = to;
    }
    if range != TimeRange::All {
        match range {
            TimeRange::Day => {
                out.start.date = date_from_chrono(now);
                out.end.date = date_from_chrono(now);
            }
            TimeRange::Week => {
                let from =
                    now - chrono::Duration::days(now.weekday().num_days_from_monday() as i64);
                out.start.date = date_from_chrono(from);
                out.end.date = date_from_chrono(from + chrono::Duration::days(7))
            }
            TimeRange::Month => {
                let from = chrono::Local.ymd(now.year(), now.month(), 1);
                // TODO Take last day of then month
                let to = from + chrono::Duration::days(30);
                out.start.date = date_from_chrono(from);
                out.end.date = date_from_chrono(to);
            }
            TimeRange::Year => {
                let from = chrono::Local.ymd(now.year(), 1, 1);
                // TODO Take last day of the year
                let to = from + chrono::Duration::days(365);
                out.start.date = date_from_chrono(from);
                out.end.date = date_from_chrono(to);
            }
            TimeRange::All => unreachable!(),
        };
    }
    out
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
