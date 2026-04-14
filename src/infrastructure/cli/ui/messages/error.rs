use crate::{
    infrastructure::cli::ui::colors::{GREEN, RED, RESET, YELLOW},
    prelude::CSTError,
};

/// Wraps a message in green for success output.
pub fn format_success(message: &str) -> String {
    GREEN.to_string() + message + RESET
}

/// Wraps a message in yellow for warning output.
pub fn format_warning(msg: &str) -> String {
    YELLOW.to_string() + msg + RESET
}

/// Formats a [`CSTError`] in red for error output.
pub fn format_error(e: &CSTError) -> String {
    RED.to_string() + &e.to_string() + RESET
}
