use crate::{
    infrastructure::io::{
        export::common::export,
        mappers::{priority_to_str, status_to_str},
    },
    prelude::{CSTError, FileFormat, Settings, Task},
};
/// Exports tasks to a Markdown file at the resolved output path.
pub fn export_markdown(
    tasks: &[Task],
    output: Option<&str>,
) -> Result<(usize, String, f64), CSTError> {
    export(tasks, output, FileFormat::Markdown, write_markdown)
}

/// Writes tasks to `path` as a Markdown table with localized headers.
fn write_markdown(tasks: &[Task], path: &str) -> Result<(), CSTError> {
    std::fs::write(path, build_markdown(tasks))?;
    Ok(())
}

/// Builds the full Markdown string: a header and one row per task.
fn build_markdown(tasks: &[Task]) -> String {
    let header = format!(
        "# {}\n\n| {} | {} | {} | {} |\n|----|------|----------|--------|",
        Settings::t("help.title"),
        Settings::t("table.id"),
        Settings::t("table.task"),
        Settings::t("table.priority"),
        Settings::t("table.status"),
    );
    let rows = tasks
        .iter()
        .map(|task| {
            format!(
                "| {:03} | {} | {} | {} |",
                task.id(),
                task.information(),
                priority_to_str(task.priority()),
                status_to_str(task.status()),
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!("{}\n{}", header, rows)
}
