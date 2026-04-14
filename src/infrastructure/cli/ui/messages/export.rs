use crate::{
    infrastructure::cli::ui::{
        colors::{CYAN, RESET},
        format_list::format_task_list,
        messages::error::format_success,
    },
    prelude::{Settings, Task},
};

/// Returns a success message with task count, elapsed time, and output path.
pub fn format_export(count: usize, path: &str, elapsed: f64) -> String {
    format_success(&format!(
        "{} {} {:.2}s → {}",
        count,
        Settings::t("ui.exported"),
        elapsed,
        path
    ))
}

/// Returns a localized pagination indicator: `Page X / Y`.
pub fn format_pagination(page: i64, page_size: i64, total: i64) -> String {
    let total_pages = (total + page_size - 1) / page_size;
    format!(
        "{}{} {} / {}{}",
        CYAN,
        Settings::t("ui.page"),
        page + 1,
        total_pages,
        RESET
    )
}

/// Formats a task list for display.
pub fn format_tasks(tasks: &[Task]) -> String {
    format_task_list(tasks)
}

/// Formats a paged task list, appending the pagination indicator if non-empty.
pub fn format_paged_tasks(tasks: &[Task], page: i64, page_size: i64, total: i64) -> String {
    let mut parts = vec![format_task_list(tasks)];
    if !tasks.is_empty() {
        parts.push(format_pagination(page, page_size, total));
    }
    parts.join("\n")
}
