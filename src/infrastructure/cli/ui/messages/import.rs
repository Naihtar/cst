use crate::prelude::{Store, ok_msg, warn_msg};

/// Returns a success message with the count of imported tasks and elapsed time.
pub fn format_import(count: usize, elapsed: f64) -> String {
    ok_msg(&format!(
        "{} {} {:.2}s",
        count,
        Store::t("ui.imported"),
        elapsed,
    ))
}

/// Returns a warning message showing how many tasks a dry-run would import.
pub fn format_import_preview(count: usize) -> String {
    warn_msg(&format!(
        "{} {} {}",
        Store::t("ui.dry_run"),
        count,
        Store::t("ui.import_preview"),
    ))
}
