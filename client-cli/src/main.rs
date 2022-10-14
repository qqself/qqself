use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use chrono::{Datelike, Local, TimeZone};
use structopt::clap::arg_enum;
use structopt::StructOpt;

use qqself_core::datetime::{Date, DateTime, DateTimeRange, DayTime};
use qqself_core::db::{Query, DB};

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

fn main() {
    let opt: Opt = Opt::from_args();
    match opt {
        Opt::Query(q) => query(q),
    }
}

fn query(opts: QueryOpts) {
    unimplemented!("TODO Whole DB goes though big rewrite currently, implement once done")
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
            // TODO Waiting for DB rewrite
        }
    }
}
