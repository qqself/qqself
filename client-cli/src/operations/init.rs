use std::{path::Path, process::exit};

use crate::key_file::KeyFile;
use qqself_core::encryption::keys::Keys;
use structopt::StructOpt;
use tracing::{error, info};

#[derive(StructOpt, Debug)]
#[structopt(about = "Creates new keys and saves it in the config.toml")]
pub struct InitOpts {
    /// Where new generated keys will be stored
    #[structopt(short, long, default_value = "qqself_keys.txt")]
    keys_path: String,

    /// If existing config file should be ignored and overwritten
    #[structopt(short, long)]
    overwrite: bool,
}

#[tracing::instrument(level = "trace", skip_all)]
pub fn init(opts: InitOpts) {
    let keys_path = Path::new(&opts.keys_path);
    if keys_path.exists() && !opts.overwrite {
        error!("Keys file already exists at {:?}", keys_path);
        exit(1);
    }
    info!(
        "Initializing. Generating new keys and storing config file at {:?}",
        keys_path
    );
    let keys = KeyFile::new(Keys::generate_new());
    keys.save(keys_path);
}
