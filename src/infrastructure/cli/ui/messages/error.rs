use crate::prelude::{
    Err, {GREEN, RED, RESET, YELLOW},
};

/// Wraps a message in green for success output.
pub fn format_success(message: &str) -> String {
    GREEN.to_string() + message + RESET
}

/// Wraps a message in yellow for warning output.
pub fn format_warning(msg: &str) -> String {
    YELLOW.to_string() + msg + RESET
}

/// Formats a [`Err`] in red for error output.
pub fn format_error(e: &Err) -> String {
    RED.to_string() + &e.to_string() + RESET
}
