use crate::prelude::{Builder, Err, IOErr};

/// Validates that `records` is non-empty and maps each entry through `parse`.
pub fn check_and_parse<T, F>(records: Vec<T>, parse: F) -> Result<Vec<Builder>, Err>
where
    F: Fn(&T) -> Result<Builder, Err>,
{
    if records.is_empty() {
        return Err(IOErr::EmptyImport.into());
    }
    records.iter().map(parse).collect()
}
