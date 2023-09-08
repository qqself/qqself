use std::{
    fs::{self},
    path::{Path, PathBuf},
    thread::{self},
};

use clap::Parser;
use qqself_core::{
    api::ApiRequest,
    binary_text::BinaryToText,
    date_time::datetime::DateDay,
    db::{Record, DB},
    encryption::{keys::PrivateKey, payload::PayloadBytes},
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
    let keys = KeyFile::load(Path::new(&opts.keys_path));
    let journal_path = Path::new(&opts.output_folder);
    let journal_path = journal_path.join(format!("qqself_journal_{}.txt", DateDay::today()));
    let wrote = download_journal(&journal_path, keys);
    info!("Downloading finished");
    wrote
}

#[tracing::instrument(level = "trace", skip_all)]
fn download_journal(journal_path: &Path, keys: KeyFile) -> PathBuf {
    info!("Downloading entries...");
    let entries = download_entries(&keys).expect("Error downloading entries");

    info!("Decrypting entries...");
    let entries = entries
        .par_lines()
        .filter_map(|line| {
            if line.is_empty() || line.starts_with('#') {
                return None;
            }
            let plain_text =
                decrypt(line.to_string(), &keys.keys().private_key).expect("Failure decrypting");
            Some(
                Record::parse(&plain_text)
                    .unwrap_or_else(|_| panic!("entry should be valid: {line}")),
            )
        })
        .collect::<Vec<_>>();

    info!("Processing entries...");
    let mut db = DB::default();
    for record in entries {
        db.add(record, false, None);
    }
    let mut output = String::new();
    let mut prev_day = None;
    db.query_results().iter().for_each(|entry| {
        if !prev_day.is_some_and(|v| v == entry.date_range().start().date()) {
            prev_day.replace(entry.date_range().start().date());
            output.push('\n');
        }
        output.push_str(&format!("{}\n", entry.to_string(true, true)));
    });
    info!("Writing entries to journal {:?} ...", journal_path);
    fs::write(journal_path, output).expect("Failed to write journal file");
    journal_path.to_owned()
}

fn download_entries(keys: &KeyFile) -> Result<String, String> {
    let (sender, mut receiver) = mpsc::channel(1);
    let keys = keys.keys().clone();
    let handle = thread::spawn(move || {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let http = Http::new();
                let body = http
                    .send(
                        ApiRequest::new_find_request(&keys, None)
                            .expect("Failed to create find API request"),
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

pub fn decrypt(data: String, private_key: &PrivateKey) -> Result<String, String> {
    let (_, payload) = data.split_once(':').expect("Expected [id]:[entry] format");
    let binary = BinaryToText::new_from_encoded(payload.to_string())
        .ok_or_else(|| "Bad data encoding".to_string())?;
    let payload = PayloadBytes::new_from_encrypted(binary).map_err(|v| v.to_string())?;
    let payload = payload.validated(None).map_err(|v| v.to_string())?;
    let decrypted = payload.decrypt(private_key).map_err(|v| v.to_string())?;
    Ok(decrypted)
}

#[cfg(test)]
mod tests {
    use qqself_core::encryption::keys::Keys;

    use super::*;

    #[test]
    fn download_operation() {
        let keys = KeyFile::new(Keys::generate_new());
        keys.save(Path::new("/tmp/keys.txt"));

        // Generate few entries, we need tokio runtime for async calls
        let handle = thread::spawn(move || {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    let http = Http::new();
                    for idx in &[1, 2, 3, 4, 5] {
                        let date = if idx > &3 { "2022-10-04" } else { "2022-10-03" };
                        let msg = format!("{date} 00:00 0{idx}:00 foo{idx}");
                        let req =
                            ApiRequest::new_set_request(keys.keys(), msg.to_string()).unwrap();
                        let resp = http.send(req).await.unwrap();
                        assert_eq!(resp.status(), 200);
                    }
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
"
        );
    }
}
