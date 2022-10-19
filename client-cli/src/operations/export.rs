use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    process::exit,
    thread::{self, JoinHandle},
};

use qqself_core::{datetime::Timestamp, encryption::payload::PayloadBytes};
use rayon::prelude::{ParallelBridge, ParallelIterator};
use structopt::StructOpt;
use tokio::sync::mpsc;
use tracing::{error, info};

use crate::{api::API, config::Config};

#[derive(StructOpt, Debug)]
#[structopt(about = "Exports all the records from journal file to the cloud")]
pub struct ExportOpts {
    /// Path to journal file with all the entries
    #[structopt(short, long, default_value = "journal.txt")]
    journal_path: String,

    /// Path to config
    #[structopt(short, long, default_value = "config.toml")]
    config_path: String,
}

// TODO I'm still not sure about error handling, but unwrap everywhere is bad. Try anyhow crate?

#[tracing::instrument(level = "trace", skip_all)]
pub fn export(opts: ExportOpts) {
    let config = Config::load(Path::new(&opts.config_path));
    let journal_path = Path::new(&opts.journal_path);
    if !journal_path.exists() {
        error!("Journal file does not exists at {:?}", journal_path);
        exit(1);
    }
    info!("Exporting. Reading journal file at {:?}", journal_path);
    export_journal(journal_path, config);
    info!("Exporting finished")
}

// For each read line we first need to encrypt it, then send to the API.  Encryption is CPU bound, so we parallel
// it with Rayon, but HTTP is async and runs on Tokio. We create N mpsc send channels to send HTTP requests in
// parallel to the backend. tokio::sync::broadcast looked like a better fit, but concept of Lagging caused issues
#[tracing::instrument(level = "trace", skip_all)]
fn export_journal(journal_path: &Path, config: Config) {
    let file = File::open(journal_path).expect("Cannot open journal file");
    let reader = BufReader::new(file);
    let (sending_runtime, send_channels) = start_sender();

    // Process all the lines in parallel using Rayon and distribute encrypted values across sending channels
    reader
        .lines()
        .enumerate()
        .par_bridge()
        .for_each(|(idx, line)| {
            let line = line.expect("Cannot read journal line");
            if line.trim().starts_with("#") {
                return; // Skip the comments
            }
            if line.trim().len() == 0 {
                return; // Skip empty lines
            }
            // Parse the record to see if it's a valid one
            qqself_core::parser::Parser::new(&line)
                .parse_date_record(None, None)
                .unwrap();
            let (public_key, private_key) = config.keys();
            let enc_bytes =
                PayloadBytes::encrypt(&public_key, &private_key, Timestamp::now(), &line, None)
                    .unwrap();
            let tx = &send_channels[idx % send_channels.len()];
            tx.blocking_send(enc_bytes).unwrap();
        });

    // Done with sending, inform receivers that we are done and wait until Tokio has finished the processing
    drop(send_channels);
    sending_runtime.join().unwrap()
}

fn start_sender() -> (JoinHandle<()>, Vec<mpsc::Sender<PayloadBytes>>) {
    let send_count = 20; // how many simultaneous requests we could have
    let mut receivers = Vec::with_capacity(send_count);
    let mut senders = Vec::with_capacity(send_count);
    for _ in 0..send_count {
        let (tx, rs) = mpsc::channel::<PayloadBytes>(1);
        senders.push(tx);
        receivers.push(rs);
    }
    // Start Tokio runtime with 1 async worker per each receiving channel
    let handle = thread::spawn(move || {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let mut join_handles = Vec::new();
                for mut rx in receivers {
                    join_handles.push(tokio::spawn(async move {
                        let api = API::new();
                        while let Some(v) = rx.recv().await {
                            let resp = api.set(v).await.unwrap();
                            if resp.status() != 200 {
                                panic!("Non 200 status"); // TODO In client-cli we don't have error handling for now, need to fix
                            }
                        }
                    }));
                }
                for handle in join_handles {
                    handle.await.unwrap();
                }
            });
    });
    (handle, senders)
}
