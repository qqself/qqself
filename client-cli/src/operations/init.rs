use std::{path::Path, process::exit};

use crate::config::Config;
use log::{error, info};
use qqself_core::encryption::keys::generate_keys;
use std::fs;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "Creates new keys and saves it in the config.toml")]
pub struct InitOpts {
    /// Where new generated config will be stored
    #[structopt(short, long, default_value = "config.toml")]
    config_path: String,

    /// If existing config file should be ignored and overwritten
    #[structopt(short, long)]
    overwrite: bool,
}

pub fn init(opts: InitOpts) {
    let config_path = Path::new(&opts.config_path);
    if config_path.exists() && !opts.overwrite {
        error!("Configuration file already exists at {:?}", config_path);
        exit(1);
    }
    info!(
        "Initializing. Generating new keys and storing config file at {:?}",
        config_path
    );
    let (public_key, private_key) = generate_keys();
    let config = Config::new(public_key.to_string(), private_key.to_string());
    let toml = toml::to_string(&config).unwrap();
    fs::write(config_path, toml).unwrap();
}
