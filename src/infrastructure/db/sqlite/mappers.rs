use crate::prelude::{Builder, Err, Task};
use rusqlite::Row;

/// Converts a SQLite row into a [`Task`] using named column access.
pub fn row_to_task(row: &Row) -> Result<Task, Err> {
    Builder::new()
        .id(row.get("id")?)
        .information(row.get("information")?)
        .priority(row.get("priority")?)
        .status(row.get("status")?)
        .build_with_id()
}
