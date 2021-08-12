use chrono::Local;

// TODO Date format doesn't belong to the client, on the
// other hand chrono likely wouldn't work in WebAssembly
pub fn time_now() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}
