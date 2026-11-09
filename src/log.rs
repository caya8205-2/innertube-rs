use serde::Serialize;
use std::fs::{self};
use std::path::PathBuf;

/// Returns the base directory for logs.
/// In development, saves to `./logs` relative to the project root.
pub fn get_log_dir() -> PathBuf {
    let current = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let log_dir = if current.join("src-tauri").exists() {
        current.join("logs")
    } else if current.ends_with("src-tauri") {
        current.join("../logs")
    } else {
        current.join("logs")
    };

    let _ = fs::create_dir_all(&log_dir);
    log_dir
}

/// Resolves the absolute path for a log file inside the `logs` folder.
pub fn resolve_log_path(filename: &str, default_ext: &str) -> PathBuf {
    let base_dir = get_log_dir();
    let file_name = if filename.contains('.') {
        filename.to_string()
    } else {
        format!("{filename}.{default_ext}")
    };
    let path = base_dir.join(file_name);
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    path
}

/// Dumps any serializable data as a pretty JSON file.
///
/// # Example
/// ```rust,no_run
/// # let raw_json = serde_json::json!({});
/// # use innertube_rs::log::dump_json;
/// dump_json("raw_youtube_feed.json", &raw_json);
/// ```
pub fn dump_json<T: Serialize + ?Sized>(filename: &str, data: &T) -> Result<PathBuf, String> {
    let file_path = resolve_log_path(filename, "json");
    let pretty_str = serde_json::to_string_pretty(data)
        .map_err(|e| format!("Failed to serialize data to JSON: {e}"))?;

    fs::write(&file_path, pretty_str)
        .map_err(|e| format!("Failed to write to file {:?}: {e}", file_path))?;

    println!("[Log] Dumped JSON: {}", file_path.display());
    Ok(file_path)
}

/// Writes any text or string slice to a file (overwrites existing file).
///
/// # Example
/// ```rust,no_run
/// # use innertube_rs::log::write_file;
/// write_file("debug.txt", "Some debug message");
/// ```
pub fn write_file(filename: &str, content: impl AsRef<str>) -> Result<PathBuf, String> {
    let file_path = resolve_log_path(filename, "log");
    fs::write(&file_path, content.as_ref())
        .map_err(|e| format!("Failed to write to file {:?}: {e}", file_path))?;

    println!("[Log] Wrote file: {}", file_path.display());
    Ok(file_path)
}

/// Dumps any data that implements `Debug` into a file.
///
/// # Example
/// ```rust,no_run
/// # use innertube_rs::log::dump_debug;
/// # let my_struct = "example";
/// dump_debug("state.txt", &my_struct);
/// ```
pub fn dump_debug<T: std::fmt::Debug + ?Sized>(
    filename: &str,
    data: &T,
) -> Result<PathBuf, String> {
    let file_path = resolve_log_path(filename, "txt");
    let content = format!("{data:#?}");
    fs::write(&file_path, content)
        .map_err(|e| format!("Failed to write to file {:?}: {e}", file_path))?;

    println!("[Log] Dumped debug: {}", file_path.display());
    Ok(file_path)
}

/// Reads key-value pairs from a `.env` file into process environment variables (`std::env`) if present.
///
/// Looks for `.env` in the current directory and its parent directories.
pub fn load_dotenv() {
    let current = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let candidates = [
        current.join(".env"),
        current.join("../.env"),
        PathBuf::from(".env"),
    ];

    for path in &candidates {
        if let Ok(content) = fs::read_to_string(path) {
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.is_empty() || trimmed.starts_with('#') {
                    continue;
                }
                if let Some((key, val)) = trimmed.split_once('=') {
                    let key = key.trim();
                    let val = val.trim().trim_matches('"').trim_matches('\'');
                    if std::env::var(key).is_err() {
                        std::env::set_var(key, val);
                    }
                }
            }
            break;
        }
    }
}
