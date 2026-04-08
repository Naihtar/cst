use crate::prelude::{
    ID, PRIORITY, STATUS, Store, Task, task_view, task_width, {CYAN, RESET},
};

/// Builds the localized column header row.
fn build_header() -> String {
    let col_task = task_width();
    format!(
        "{}{:<ID$} | {:<col_task$} | {:<PRIORITY$} | {:<STATUS$}{}",
        CYAN,
        Store::t("table.id"),
        Store::t("table.task"),
        Store::t("table.priority"),
        Store::t("table.status"),
        RESET
    )
}

/// Builds the separator line sized to match the table width.
fn build_separator() -> String {
    let col_task = task_width();
    let width = ID + 3 + col_task + 3 + PRIORITY + 3 + STATUS + 3;
    format!("{}{}{}", CYAN, "-".repeat(width), RESET)
}

/// Formats a slice of tasks as an aligned table with header and separator.
///
/// Returns a "no tasks found" message if the slice is empty.
pub fn format_task_list(tasks: &[Task]) -> String {
    if tasks.is_empty() {
        return CYAN.to_string() + &Store::t("ui.no_tasks_found") + RESET;
    }
    [build_header(), build_separator()]
        .into_iter()
        .chain(tasks.iter().map(task_view))
        .collect::<Vec<_>>()
        .join("\n")
}
