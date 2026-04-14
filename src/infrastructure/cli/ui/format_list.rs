use crate::{
    infrastructure::cli::ui::{
        colors::{CYAN, RESET},
        format_task::{COL_ID, COL_PRIORITY, COL_STATUS, col_task, format_task},
    },
    prelude::{Settings, Task},
};

/// Builds the localized column header row.
fn build_header() -> String {
    let col_task = col_task();
    format!(
        "{}{:<COL_ID$} | {:<col_task$} | {:<COL_PRIORITY$} | {:<COL_STATUS$}{}",
        CYAN,
        Settings::t("table.id"),
        Settings::t("table.task"),
        Settings::t("table.priority"),
        Settings::t("table.status"),
        RESET
    )
}

/// Builds the separator line sized to match the table width.
fn build_separator() -> String {
    let col_task = col_task();
    let width = COL_ID + 3 + col_task + 3 + COL_PRIORITY + 3 + COL_STATUS + 3;
    format!("{}{}{}", CYAN, "-".repeat(width), RESET)
}

/// Formats a slice of tasks as an aligned table with header and separator.
///
/// Returns a "no tasks found" message if the slice is empty.
pub fn format_task_list(tasks: &[Task]) -> String {
    if tasks.is_empty() {
        return CYAN.to_string() + &Settings::t("ui.no_tasks_found") + RESET;
    }
    [build_header(), build_separator()]
        .into_iter()
        .chain(tasks.iter().map(format_task))
        .collect::<Vec<_>>()
        .join("\n")
}
