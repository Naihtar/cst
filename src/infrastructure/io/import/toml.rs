use crate::prelude::{Builder, Err, IOErr, check_and_parse, str_to_priority, str_to_status};

use serde::Deserialize;

/// Deserialization wrapper for the top-level `[tasks]` TOML array.
#[derive(Deserialize)]
struct Root {
    tasks: Vec<TaskRecord>,
}

/// Deserialization view of a single task entry in the TOML file.
#[derive(Deserialize)]
struct TaskRecord {
    information: String,
    priority: Option<String>,
    status: Option<String>,
}

/// Imports tasks from a TOML file structured as `[[tasks]]` entries.
pub fn import_toml(path: &str) -> Result<Vec<Builder>, Err> {
    let root: Root = toml::from_str(&std::fs::read_to_string(path)?)
        .map_err(|e| IOErr::ParseError(e.to_string()))?;
    check_and_parse(root.tasks, parse_record)
}

/// Parses a single TOML task record into a [`Builder`].
///
/// Returns an error if `information` is blank.
fn parse_record(record: &TaskRecord) -> Result<Builder, Err> {
    if record.information.trim().is_empty() {
        Err(IOErr::MissingField("information".to_string()))?;
    }
    Ok(Builder::new()
        .information(record.information.clone())
        .priority(record.priority.as_deref().and_then(str_to_priority))
        .status(record.status.as_deref().and_then(str_to_status)))
}
