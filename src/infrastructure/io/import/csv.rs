use crate::{
    domain::models::task::entity::TaskBuilder,
    infrastructure::io::mappers::{str_to_priority, str_to_status},
    prelude::{CSTError, IOErr},
};

/// Imports tasks from a CSV file, auto-detecting `,` or `;` as delimiter.
///
/// The first column must be `information`. `priority` and `status` are optional.
pub fn import_csv(path: &str) -> Result<Vec<TaskBuilder>, CSTError> {
    let content = std::fs::read_to_string(path)?;
    let mut lines = content.lines();
    lines.next();
    if lines.next().is_none() {
        return Err(IOErr::EmptyImport.into());
    }
    let delimiter = detect_delimiter(path)?;
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .from_path(path)?;
    let headers = reader.headers()?;
    if headers.get(0) != Some("information") {
        Err(IOErr::MissingField("information".to_string()))?;
    }
    reader
        .records()
        .map(|result| parse_record(&result?))
        .collect()
}

/// Reads the first line of the file to detect whether it uses `;` or `,`.
fn detect_delimiter(path: &str) -> Result<u8, CSTError> {
    let content = std::fs::read_to_string(path)?;
    Ok(match content.lines().next() {
        Some(line) if line.contains(';') => b';',
        _ => b',',
    })
}

/// Parses a single CSV record into a [`TaskBuilder`].
fn parse_record(record: &csv::StringRecord) -> Result<TaskBuilder, CSTError> {
    let information = record
        .get(0)
        .ok_or(IOErr::MissingField("information".to_string()))?;
    Ok(TaskBuilder::new()
        .information(information.to_string())
        .priority(record.get(1).and_then(str_to_priority))
        .status(record.get(2).and_then(str_to_status)))
}
