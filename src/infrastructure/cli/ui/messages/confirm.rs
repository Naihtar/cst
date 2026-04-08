use crate::prelude::{RESET, Store, YELLOW};

/// User response to a confirmation prompt.
#[derive(Debug)]
pub enum Decision {
    Yes,
    No,
}

/// Wraps a confirmation message in yellow ANSI color.
fn format_confirm(message: &str) -> String {
    YELLOW.to_string() + message + RESET
}

/// Returns a confirmation prompt for removing a single task by ID.
pub fn format_confirm_remove(id: i64) -> String {
    format_confirm(&Store::t("ui.are_you_sure_remove").replace("{0}", &format!("{:03}", id)))
}

/// Returns a confirmation prompt for removing multiple tasks by ID.
pub fn format_confirm_remove_many(ids: &[i64]) -> String {
    let ids_str = ids
        .iter()
        .map(|id| id.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    format_confirm(&Store::t("ui.are_you_sure_remove_many").replace("{0}", &ids_str))
}

/// Returns a confirmation prompt for clearing all tasks.
pub fn format_confirm_clear() -> String {
    format_confirm(&Store::t("ui.are_you_sure_clear"))
}

/// Returns a confirmation prompt for appending tasks from `path`.
pub fn format_confirm_import(path: &str) -> String {
    format_confirm(&Store::t("ui.are_you_sure_import").replace("{0}", path))
}

/// Returns a confirmation prompt for restoring tasks from `path`.
pub fn format_confirm_restore(path: &str) -> String {
    format_confirm(&Store::t("ui.are_you_sure_restore").replace("{0}", path))
}
