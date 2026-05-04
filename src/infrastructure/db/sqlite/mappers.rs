use crate::prelude::{CSTError, Task, TaskBuilder};
use rusqlite::Row;

/// Converts a SQLite row into a [`Task`] using named column access.
pub fn row_to_task(row: &Row) -> Result<Task, CSTError> {
    TaskBuilder::new()
        .id(row.get("id")?)
        .information(row.get("information")?)
        .priority(row.get("priority")?)
        .status(row.get("status")?)
        .build_with_id()
}
