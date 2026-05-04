use crate::{
    infrastructure::cli::ui::messages::error::{format_success, format_warning},
    prelude::Settings,
};

/// Returns a success message with the count of imported tasks and elapsed time.
pub fn format_import(count: usize, elapsed: f64) -> String {
    format_success(&format!(
        "{} {} {:.2}s",
        count,
        Settings::t("ui.imported"),
        elapsed,
    ))
}

/// Returns a warning message showing how many tasks a dry-run would import.
pub fn format_import_preview(count: usize) -> String {
    format_warning(&format!(
        "{} {} {}",
        Settings::t("ui.dry_run"),
        count,
        Settings::t("ui.import_preview"),
    ))
}
