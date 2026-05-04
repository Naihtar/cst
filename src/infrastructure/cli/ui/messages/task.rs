use std::slice::from_ref;

use crate::{
    infrastructure::cli::ui::{
        colors::{RED, RESET},
        format_list::format_task_list,
        messages::error::{format_success, format_warning},
    },
    prelude::{Settings, Task},
};

/// Formats a single task as a one-row table for display.
pub fn format_task_found(task: &Task) -> String {
    format_task_list(from_ref(task))
}

/// Returns a success message with the ID of the newly created task.
pub fn format_task_created(id: i64) -> String {
    format_success(&format!("{} #{:03}", Settings::t("ui.task_created"), id))
}

/// Returns a success message with the ID of the updated task.
pub fn format_task_updated(id: i64) -> String {
    format_success(&format!("{} #{:03}", Settings::t("ui.task_updated"), id))
}

/// Returns a success message with the ID of the deleted task.
pub fn format_task_deleted(id: i64) -> String {
    format_success(&format!("{} #{:03}", Settings::t("ui.task_deleted"), id))
}

/// Returns a generic done success message.
pub fn format_done() -> String {
    format_success(&Settings::t("ui.done"))
}

/// Returns a red-colored cancellation message.
pub fn format_cancelled_msg() -> String {
    RED.to_string() + &Settings::t("ui.cancelled") + RESET
}

/// Formats a comma-separated string from a slice of IDs.
fn ids_to_str(ids: &[i64]) -> String {
    ids.iter()
        .map(|id| id.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

/// Returns processed and missing ID lists as a combined success/warning message.
///
/// Shows only the warning if nothing was processed, only the success if nothing
/// was missing, or both joined by a newline otherwise.
pub fn format_many_result(processed: &[i64], missing: &[i64]) -> String {
    let done = format_success(&format!(
        "{}: {}",
        Settings::t("ui.ids_processed"),
        ids_to_str(processed)
    ));

    if missing.is_empty() {
        done
    } else if processed.is_empty() {
        format_warning(&format!(
            "{}: {}",
            Settings::t("ui.ids_not_found"),
            ids_to_str(missing)
        ))
    } else {
        format!(
            "{}\n{}",
            done,
            format_warning(&format!(
                "{}: {}",
                Settings::t("ui.ids_not_found"),
                ids_to_str(missing)
            ))
        )
    }
}
