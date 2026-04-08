use crate::prelude::{Builder, Err, IOErr, check_and_parse, str_to_priority, str_to_status};

/// Imports tasks from a JSON array file.
///
/// Each object must have an `information` field. `priority` and `status` are optional.
pub fn import_json(path: &str) -> Result<Vec<Builder>, Err> {
    let records: Vec<serde_json::Value> = serde_json::from_str(&std::fs::read_to_string(path)?)?;
    check_and_parse(records, parse_record)
}

/// Parses a single JSON object into a [`Builder`].
fn parse_record(record: &serde_json::Value) -> Result<Builder, Err> {
    let information = record
        .get("information")
        .and_then(|v| v.as_str())
        .ok_or_else(|| IOErr::MissingField("information".to_string()))?;
    Ok(Builder::new()
        .information(information.to_string())
        .priority(
            record
                .get("priority")
                .and_then(|v| v.as_str())
                .and_then(str_to_priority),
        )
        .status(
            record
                .get("status")
                .and_then(|v| v.as_str())
                .and_then(str_to_status),
        ))
}
