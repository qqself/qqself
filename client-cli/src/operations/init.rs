use std::{path::Path, process::exit};

use crate::config::Config;
use qqself_core::encryption::keys::{generate_keys, PrivateKey, PublicKey};
use structopt::StructOpt;
use tracing::{error, info};

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

#[tracing::instrument(level = "trace", skip_all)]
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
    let (public_key, private_key) = generate_new_keys();
    let config = Config::new(public_key, private_key);
    config.save(config_path);
}

#[tracing::instrument(level = "trace", skip_all)]
fn generate_new_keys() -> (PublicKey, PrivateKey) {
    generate_keys()
}
