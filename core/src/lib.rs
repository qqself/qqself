pub mod api;
pub mod binary_text;
pub mod data_views;
pub mod date_time;
pub mod db;
pub mod encryption;
pub mod parsing;
pub mod progress;
pub mod record;

/// Returns compile time build info
pub fn build_info() -> String {
    [
        ("Build", env!("VERGEN_BUILD_TIMESTAMP")),
        ("Commit", env!("VERGEN_GIT_COMMIT_MESSAGE")),
        ("Hash", env!("VERGEN_GIT_SHA")),
        ("Host", env!("VERGEN_RUSTC_HOST_TRIPLE")),
        ("Profile", env!("VERGEN_CARGO_OPT_LEVEL")),
        ("Rust", env!("VERGEN_RUSTC_SEMVER")),
        ("Target", env!("VERGEN_CARGO_TARGET_TRIPLE")),
    ]
    .map(|(k, v)| format!("{k}: {v}"))
    .join("\n")
}
