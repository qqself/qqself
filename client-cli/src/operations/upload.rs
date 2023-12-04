use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    process::exit,
    thread::{self, JoinHandle},
};

use clap::Parser;
use qqself_core::{api::{Request, ApiRequests}, record::Entry, encryption::cryptor::Cryptor};
use rayon::prelude::{ParallelBridge, ParallelIterator};
use tokio::sync::mpsc;
use tracing::{error, info};

use crate::{http::Http, key_file::KeyFile};

#[derive(Parser, Debug)]
#[command(about = "Uploads all the records from journal file to the server")]
pub struct UploadOpts {
    /// Path to journal file with all the entries
    #[arg(short, long, default_value = "journal.txt")]
    journal_path: String,

    /// Path to key file
    #[arg(short, long, default_value = "qqself_keys.txt")]
    keys_path: String,
}

// TODO I'm still not sure about error handling, but unwrap everywhere is bad. Try anyhow crate?

#[tracing::instrument(level = "trace", skip_all)]
pub fn upload(opts: UploadOpts) {
    info!("Uploading. Reading key file at {:?}", opts.keys_path);
    let keys = KeyFile::load_from_file(Path::new(&opts.keys_path));
    let journal_path = Path::new(&opts.journal_path);
    if !journal_path.exists() {
        error!("Journal file does not exists at {:?}", journal_path);
        exit(1);
    }
    info!("Uploading. Reading journal file at {:?}", journal_path);
    upload_journal(journal_path, keys.cryptor());
    info!("Uploading finished")
}

// For each read line we first need to encrypt it, then send to the API.  Encryption is CPU bound, so we parallel
// it with Rayon, but HTTP is async and runs on Tokio. We create N mpsc send channels to send HTTP requests in
// parallel to the backend. tokio::sync::broadcast looked like a better fit, but concept of Lagging caused issues
#[tracing::instrument(level = "trace", skip_all)]
fn upload_journal(journal_path: &Path, cryptor: Cryptor) {
    let file = File::open(journal_path).expect("Cannot open journal file");
    let reader = BufReader::new(file);
    let (sending_runtime, send_channels) = start_sender();
    let api = ApiRequests::default();

    // Process all the lines in parallel using Rayon and distribute encrypted values across sending channels
    reader
        .lines()
        .enumerate()
        .par_bridge()
        .for_each(|(idx, line)| {
            let line = line.expect("Cannot read journal line");
            if line.trim().starts_with('#') {
                return; // Skip the comments
            }
            if line.trim().is_empty() {
                return; // Skip empty lines
            }
            // Parse the record to see if it's a valid one
            if let Err(err) = Entry::parse(&line) {
                panic!("Error {} parsing line: {}", err, &line);
            }
            let payload = cryptor.encrypt(&line).expect("Failure to encrypt");
            let req = api.create_set_request(payload);
            let tx = &send_channels[idx % send_channels.len()];
            tx.blocking_send(req).unwrap();
        });

    // Done with sending, inform receivers that we are done and wait until Tokio has finished the processing
    drop(send_channels);
    sending_runtime.join().unwrap()
}

fn start_sender() -> (JoinHandle<()>, Vec<mpsc::Sender<Request>>) {
    let send_count = 20; // how many simultaneous requests we could have
    let mut receivers = Vec::with_capacity(send_count);
    let mut senders = Vec::with_capacity(send_count);
    for _ in 0..send_count {
        let (tx, rs) = mpsc::channel::<Request>(1);
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
                        let http = Http::new();
                        while let Some(v) = rx.recv().await {
                            let resp = http.send(v).await.unwrap();
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
