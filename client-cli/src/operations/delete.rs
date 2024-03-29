use std::{io, path::Path, thread};

use clap::Parser;
use qqself_core::{api::ApiRequests, encryption::cryptor::Cryptor};

use tracing::{info, warn};

use crate::{http::Http, key_file::KeyFile};

#[derive(Parser, Debug)]
#[command(about = "Delete all the records from the server")]
pub struct DeleteOpts {
    /// Path to key file
    #[arg(short, long, default_value = "qqself_keys.txt")]
    keys_path: String,
}

#[tracing::instrument(level = "trace", skip_all)]
pub fn delete(opts: DeleteOpts) {
    info!(
        "Deleting all the entries. Reading key file at {:?}",
        opts.keys_path
    );
    let keys = KeyFile::load_from_file(Path::new(&opts.keys_path));

    warn!("All records will be deleted from the server, to proceed type DELETE or anything else to cancel");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let confirmation_string = "DELETE\n";
    if input != confirmation_string {
        println!("Cancelled");
    }
    delete_entries(keys.cryptor());
    info!("All records deleted")
}

#[tracing::instrument(level = "trace", skip_all)]
fn delete_entries(cryptor: Cryptor) {
    let handle = thread::spawn(move || {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let api = ApiRequests::default();
                let req = api.create_delete_request(cryptor.sign_delete_token().unwrap());
                let http = Http::new();
                let resp = http.send(req).await.unwrap();
                if resp.status() != 200 {
                    panic!("Non 200 status");
                }
            });
    });
    handle.join().unwrap();
}
