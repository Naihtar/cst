use crate::{
    infrastructure::io::{
        export::common::export,
        mappers::{priority_to_str, status_to_str},
    },
    prelude::{CSTError, FileFormat, Task},
};

/// Exports tasks to a pretty-printed JSON file at the resolved output path.
pub fn export_json(tasks: &[Task], output: Option<&str>) -> Result<(usize, String, f64), CSTError> {
    export(tasks, output, FileFormat::Json, write_json)
}

/// Serializes tasks to a JSON array and writes it to `path`.
fn write_json(tasks: &[Task], path: &str) -> Result<(), CSTError> {
    let records: Vec<_> = tasks
        .iter()
        .map(|task| {
            serde_json::json!({
                "id": task.id(),
                "information": task.information(),
                "priority": priority_to_str(task.priority()),
                "status": status_to_str(task.status()),
            })
        })
        .collect();
    let content = serde_json::to_string_pretty(&records)?;
    std::fs::write(path, content)?;
    Ok(())
}
