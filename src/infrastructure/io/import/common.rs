use crate::prelude::{CSTError, IOErr, TaskBuilder};

/// Validates that `records` is non-empty and maps each entry through `parse`.
pub fn check_and_parse<T, F>(records: Vec<T>, parse: F) -> Result<Vec<TaskBuilder>, CSTError>
where
    F: Fn(&T) -> Result<TaskBuilder, CSTError>,
{
    if records.is_empty() {
        return Err(IOErr::EmptyImport.into());
    }
    records.iter().map(parse).collect()
}
