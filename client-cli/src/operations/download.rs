use std::{
    fs::{self},
    path::{Path, PathBuf},
    thread::{self},
};

use clap::Parser;
use qqself_core::{
    api::ApiRequests,
    date_time::datetime::DateDay,
    db::Record,
    encryption::cryptor::Cryptor,
};
use rayon::prelude::*;
use rayon::str::ParallelString;
use tokio::sync::mpsc;
use tracing::info;

use crate::{http::Http, key_file::KeyFile};

#[derive(Parser, Debug)]
#[command(about = "Download all the entries from the server to the file")]
pub struct DownloadOpts {
    /// Path to folder where journal will be created with the name format of `qqself_journal_[TODAY].txt`
    #[arg(short, long, default_value = ".")]
    output_folder: String,

    /// Path to key file
    #[arg(short, long, default_value = "qqself_keys.txt")]
    keys_path: String,
}

#[tracing::instrument(level = "trace", skip_all)]
pub fn download(opts: DownloadOpts) -> PathBuf {
    info!("Downloading. Reading key file at {:?}", opts.keys_path);
    let keys = KeyFile::load_from_file(Path::new(&opts.keys_path));
    let journal_path = Path::new(&opts.output_folder);
    let journal_path = journal_path.join(format!("qqself_journal_{}.txt", DateDay::today()));
    let wrote = download_journal(&journal_path, keys.cryptor());
    info!("Downloading finished");
    wrote
}

#[tracing::instrument(level = "trace", skip_all)]
fn download_journal(journal_path: &Path, cryptor: Cryptor) -> PathBuf {
    info!("Downloading entries...");
    let entries = download_entries(cryptor.clone()).expect("Error downloading entries");

    info!("Decrypting entries...");
    let entries = entries
        .par_lines()
        .filter_map(|line| {
            if line.is_empty() || line.starts_with('#') {
                return None;
            }
            let (_, payload) = line.split_once(':').expect("Expected [id]:[entry] format");
            let plain_text = cryptor
                .decrypt(payload.to_string())
                .expect("Failure decrypting");
            Some(
                Record::parse(&plain_text)
                    .unwrap_or_else(|_| panic!("entry should be valid: {line}")),
            )
        })
        .collect::<Vec<_>>();

    info!("Processing entries...");
    let mut output = String::new();
    let mut prev_day = None;
    for entry in entries {
        if !prev_day.is_some_and(|v| v == entry.date_range().start().date()) {
            prev_day.replace(entry.date_range().start().date());
            output.push('\n');
        }
        output.push_str(&format!("{}\n", entry.to_string(true, true)));
    }
    info!("Writing entries to journal {:?} ...", journal_path);
    fs::write(journal_path, output).expect("Failed to write journal file");
    journal_path.to_owned()
}

fn download_entries(cryptor: Cryptor) -> Result<String, String> {
    let (sender, mut receiver) = mpsc::channel(1);
    let handle = thread::spawn(move || {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let http = Http::new();
                let api = ApiRequests::default();
                let body = http
                    .send(
                        api.create_find_request(
                            cryptor
                                .sign_find_token(None)
                                .expect("Failed to create find API request"),
                        ),
                    )
                    .await;
                match body {
                    Ok(resp) => match resp.text().await {
                        Ok(text) => sender.send(Ok(text)),
                        Err(err) => sender.send(Err(err.to_string())),
                    },
                    Err(err) => sender.send(Err(err.to_string())),
                }
                .await
            })
    });
    _ = handle.join().expect("Error downloading");
    match receiver.blocking_recv() {
        Some(resp) => resp,
        None => panic!("Failed to get a response"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn download_operation() {
        let keys = KeyFile::generate_new();
        keys.save_to_file(Path::new("/tmp/keys.txt"));

        // Generate few entries, we need tokio runtime for async calls
        let handle = thread::spawn(move || {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    let http = Http::new();
                    let api = ApiRequests::default();
                    let cryptor = keys.cryptor();
                    for idx in &[1, 2, 3, 4, 5] {
                        let date = if idx > &3 { "2022-10-04" } else { "2022-10-03" };
                        let msg = format!("{date} 00:00 0{idx}:00 foo{idx}");
                        let payload = cryptor.encrypt(&msg).unwrap();
                        let req = api.create_set_request(payload);
                        let resp = http.send(req).await.unwrap();
                        assert_eq!(resp.status(), 200);
                    }
                    // Overwrite last message to ensure both previous and new record are preserved
                    let msg = "2022-10-04 00:00 05:00 updated. entry revision=2";
                    let payload = cryptor.encrypt(msg).unwrap();
                    let req = api.create_set_request(payload);
                    let resp = http.send(req).await.unwrap();
                    assert_eq!(resp.status(), 200);
                })
        });
        handle.join().unwrap();

        // Check for the output
        let wrote = download(DownloadOpts {
            output_folder: "/tmp".to_string(),
            keys_path: "/tmp/keys.txt".to_string(),
        });
        let content = fs::read_to_string(wrote).unwrap();
        assert_eq!(
            content,
            "
2022-10-03 00:00 01:00 foo1
2022-10-03 00:00 02:00 foo2
2022-10-03 00:00 03:00 foo3

2022-10-04 00:00 04:00 foo4
2022-10-04 00:00 05:00 foo5
2022-10-04 00:00 05:00 updated. entry revision=2
"
        );
    }
}
