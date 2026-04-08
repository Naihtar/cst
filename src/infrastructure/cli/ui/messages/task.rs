use std::slice::from_ref;

use crate::prelude::{
    Store, Task, list_view, ok_msg, warn_msg, {RED, RESET},
};

/// Returns a success message confirming all tasks were cleared, with the backup path.
pub fn format_cleared(backup_path: &str) -> String {
    ok_msg(&Store::t("ui.cleared").replace("{0}", backup_path))
}

/// Formats a single task as a one-row table for display.
pub fn format_task_found(task: &Task) -> String {
    list_view(from_ref(task))
}

/// Returns a success message with the ID of the newly created task.
pub fn format_task_created(id: i64) -> String {
    ok_msg(&format!("{} #{:03}", Store::t("ui.task_created"), id))
}

/// Returns a success message with the ID of the updated task.
pub fn format_task_updated(id: i64) -> String {
    ok_msg(&format!("{} #{:03}", Store::t("ui.task_updated"), id))
}

/// Returns a success message with the ID of the deleted task.
pub fn format_task_deleted(id: i64) -> String {
    ok_msg(&format!("{} #{:03}", Store::t("ui.task_deleted"), id))
}

/// Returns a generic done success message.
pub fn format_done() -> String {
    ok_msg(&Store::t("ui.done"))
}

/// Returns a red-colored cancellation message.
pub fn format_cancelled_msg() -> String {
    RED.to_string() + &Store::t("ui.cancelled") + RESET
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
    let done = ok_msg(&format!(
        "{}: {}",
        Store::t("ui.ids_processed"),
        ids_to_str(processed)
    ));

    if missing.is_empty() {
        done
    } else if processed.is_empty() {
        warn_msg(&format!(
            "{}: {}",
            Store::t("ui.ids_not_found"),
            ids_to_str(missing)
        ))
    } else {
        format!(
            "{}\n{}",
            done,
            warn_msg(&format!(
                "{}: {}",
                Store::t("ui.ids_not_found"),
                ids_to_str(missing)
            ))
        )
    }
}
