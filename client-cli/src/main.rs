use operations::{
    export::{export, ExportOpts},
    init::{init, InitOpts},
    report::{report, ReportOpts},
};
use structopt::StructOpt;
use tracing::metadata::LevelFilter;
use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};
mod api;
mod config;
mod operations;

#[derive(StructOpt, Debug)]
enum Opts {
    Init(InitOpts),
    Export(ExportOpts),
    Report(ReportOpts),
}

fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_span_events(FmtSpan::CLOSE)
        .with_timer(tracing_subscriber::fmt::time::LocalTime::rfc_3339())
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();
    match Opts::from_args() {
        Opts::Init(opts) => init(opts),
        Opts::Export(opts) => export(opts),
        Opts::Report(opts) => report(opts),
    }
}
