use operations::{
    download::{download, DownloadOpts},
    init::{init, InitOpts},
    report::{report, ReportOpts},
    upload::{upload, UploadOpts},
};
use structopt::StructOpt;
use tracing::metadata::LevelFilter;
use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};
mod http;
mod key_file;
mod operations;

#[derive(StructOpt, Debug)]
enum Opts {
    Init(InitOpts),
    Upload(UploadOpts),
    Download(DownloadOpts),
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
        Opts::Upload(opts) => upload(opts),
        Opts::Report(opts) => report(opts),
        Opts::Download(opts) => {
            download(opts);
        }
    }
}
