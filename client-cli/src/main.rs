use env_logger::Env;
use operations::init::{init, InitOpts};
use structopt::StructOpt;
mod config;
mod operations;

#[derive(StructOpt, Debug)]
enum Opts {
    Init(InitOpts),
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format_target(false)
        .init();
    match Opts::from_args() {
        Opts::Init(opts) => init(opts),
    }
}
