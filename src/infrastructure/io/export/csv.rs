use crate::{
    infrastructure::io::{
        export::common::export,
        mappers::{priority_to_str, status_to_str},
    },
    prelude::{CSTError, FileFormat, Task},
};

/// Exports tasks to a CSV file at the resolved output path.
pub fn export_csv(tasks: &[Task], output: Option<&str>) -> Result<(usize, String, f64), CSTError> {
    export(tasks, output, FileFormat::Csv, write_csv)
}

/// Writes tasks to `path` as comma-delimited CSV with a header row.
fn write_csv(tasks: &[Task], path: &str) -> Result<(), CSTError> {
    let mut writer = csv::WriterBuilder::new().delimiter(b',').from_path(path)?;
    writer.write_record(["information", "priority", "status"])?;
    for task in tasks {
        writer.write_record([
            task.information(),
            priority_to_str(task.priority()),
            status_to_str(task.status()),
        ])?;
    }
    Ok(writer.flush()?)
}
