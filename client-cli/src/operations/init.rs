use std::{path::Path, process::exit};

use crate::key_file::KeyFile;
use clap::Parser;
use tracing::{error, info};

#[derive(Parser, Debug)]
#[command(about = "Creates new key file")]
pub struct InitOpts {
    /// Where new generated keys will be stored
    #[arg(short, long, default_value = "qqself_keys.txt")]
    keys_path: String,

    /// If existing config file should be ignored and overwritten
    #[arg(short, long)]
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
        "Initializing. Generating new keys and storing it at {:?}",
        keys_path
    );
    let keys = KeyFile::generate_new();
    keys.save_to_file(keys_path);
}
