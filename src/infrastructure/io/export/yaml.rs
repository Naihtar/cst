use crate::{
    infrastructure::io::{
        export::common::export,
        mappers::{priority_to_str, status_to_str},
    },
    prelude::{CSTError, FileFormat, Task},
};
use serde::Serialize;

/// Serialization view of a single task for YAML output.
#[derive(Serialize)]
struct TaskRecord<'a> {
    id: i64,
    information: &'a str,
    priority: &'a str,
    status: &'a str,
}

/// Exports tasks to a YAML file at the resolved output path.
pub fn export_yaml(tasks: &[Task], output: Option<&str>) -> Result<(usize, String, f64), CSTError> {
    export(tasks, output, FileFormat::Yaml, write_yaml)
}

/// Serializes tasks as a YAML sequence and writes it to `path`.
fn write_yaml(tasks: &[Task], path: &str) -> Result<(), CSTError> {
    let records: Vec<TaskRecord> = tasks
        .iter()
        .map(|task| TaskRecord {
            id: task.id(),
            information: task.information(),
            priority: priority_to_str(task.priority()),
            status: status_to_str(task.status()),
        })
        .collect();
    let content = serde_yaml::to_string(&records)?;
    std::fs::write(path, content)?;
    Ok(())
}
