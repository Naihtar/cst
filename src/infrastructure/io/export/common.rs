use crate::{
    infrastructure::io::interactor::ask_filename,
    prelude::{CSTError, FileFormat, IOErr, Progress, Task},
};

use std::path::PathBuf;

/// Resolves the output path, runs `write`, and returns `(count, path, elapsed_secs)`.
///
/// If `output` is `None` or a directory, prompts the user for a filename.
pub fn export<F>(
    tasks: &[Task],
    output: Option<&str>,
    format: FileFormat,
    write: F,
) -> Result<(usize, String, f64), CSTError>
where
    F: Fn(&[Task], &str) -> Result<(), CSTError>,
{
    if tasks.is_empty() {
        Err(IOErr::EmptyExport)?;
    }
    let path = resolve_output_path(output, format)?;
    let progress = Progress::new();
    write(tasks, &path)?;
    Ok((tasks.len(), path, progress.elapsed_secs()))
}

/// Returns the output path as a string.
///
/// Uses `output` directly if it has a known extension, prompts within `output`
/// as directory otherwise, or falls back to the OS documents directory.
fn resolve_output_path(output: Option<&str>, format: FileFormat) -> Result<String, CSTError> {
    match output {
        Some(path) if has_known_extension(path) => Ok(path.to_string()),
        Some(dir) => ask_filename(format, &PathBuf::from(dir)),
        None => ask_filename(format, &default_dir()),
    }
}

/// Returns `true` if `path` ends with a supported export extension.
fn has_known_extension(path: &str) -> bool {
    matches!(
        std::path::Path::new(path)
            .extension()
            .and_then(|e| e.to_str()),
        Some("csv" | "md" | "xlsx" | "json" | "yml" | "yaml" | "toml")
    )
}

/// Returns the OS documents directory, falling back to the current directory.
fn default_dir() -> PathBuf {
    dirs::document_dir().unwrap_or_else(|| PathBuf::from("."))
}
