use crate::prelude::{Err, FileFormat, IOErr, Task, export, priority_to_str, status_to_str};

use serde::Serialize;

/// Serialization view of a single task for TOML output.
#[derive(Serialize)]
struct TaskRecord<'a> {
    id: i64,
    information: &'a str,
    priority: &'a str,
    status: &'a str,
}

/// Top-level TOML structure wrapping the task list under a `[tasks]` key.
#[derive(Serialize)]
struct Root<'a> {
    tasks: Vec<TaskRecord<'a>>,
}

/// Exports tasks to a TOML file at the resolved output path.
pub fn export_toml(tasks: &[Task], output: Option<&str>) -> Result<(usize, String, f64), Err> {
    export(tasks, output, FileFormat::Toml, write_toml)
}

/// Serializes tasks under a `[[tasks]]` array and writes it to `path`.
fn write_toml(tasks: &[Task], path: &str) -> Result<(), Err> {
    let records: Vec<TaskRecord> = tasks
        .iter()
        .map(|task| TaskRecord {
            id: task.id(),
            information: task.information(),
            priority: priority_to_str(task.priority()),
            status: status_to_str(task.status()),
        })
        .collect();
    let content = toml::to_string_pretty(&Root { tasks: records })
        .map_err(|e| IOErr::SerializationError(e.to_string()))?;
    std::fs::write(path, content)?;
    Ok(())
}
