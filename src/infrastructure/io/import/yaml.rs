use crate::{
    infrastructure::io::{
        import::common::check_and_parse,
        mappers::{str_to_priority, str_to_status},
    },
    prelude::{CSTError, IOErr, TaskBuilder},
};

/// Imports tasks from a YAML sequence file.
///
/// Each entry must have an `information` field. `priority` and `status` are optional.
pub fn import_yaml(path: &str) -> Result<Vec<TaskBuilder>, CSTError> {
    let records: Vec<serde_yaml::Value> = serde_yaml::from_str(&std::fs::read_to_string(path)?)?;
    check_and_parse(records, parse_record)
}

/// Parses a single YAML mapping into a [`TaskBuilder`].
fn parse_record(record: &serde_yaml::Value) -> Result<TaskBuilder, CSTError> {
    let information = record
        .get("information")
        .and_then(|v| v.as_str())
        .ok_or_else(|| IOErr::MissingField("information".to_string()))?;
    Ok(TaskBuilder::new()
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
