use std::io::Write;

use crate::{
    infrastructure::cli::{
        mappers::accept,
        ui::messages::confirm::{
            Decision, format_confirm_clear, format_confirm_import, format_confirm_remove,
            format_confirm_remove_many, format_confirm_restore,
        },
    },
    prelude::{CSTError, CliErr},
};

/// Prints `message`, reads a line from stdin, and parses it as a [`Decision`].
fn ask(message: &str) -> Result<Decision, CSTError> {
    print!("{}", message);
    std::io::stdout()
        .flush()
        .map_err(|_| CliErr::StdoutFlushError)?;
    let line = std::io::stdin()
        .lines()
        .next()
        .ok_or(CliErr::EmptyInputStream)?
        .map_err(|_| CliErr::StdinReadError)?;
    accept(line.trim())
}

/// Asks the user to confirm a restore-mode import for the given path.
pub fn ask_confirmation_restore(path: &str) -> Result<Decision, CSTError> {
    ask(&format_confirm_restore(path))
}

/// Asks the user to confirm deletion of the task with the given ID.
pub fn ask_confirmation_remove(id: i64) -> Result<Decision, CSTError> {
    ask(&format_confirm_remove(id))
}

/// Asks the user to confirm deletion of multiple tasks.
pub fn ask_confirmation_remove_many(ids: &[i64]) -> Result<Decision, CSTError> {
    ask(&format_confirm_remove_many(ids))
}

/// Asks the user to confirm clearing all tasks.
pub fn ask_confirmation_clear() -> Result<Decision, CSTError> {
    ask(&format_confirm_clear())
}

/// Asks the user to confirm an append-mode import for the given path.
pub fn ask_confirmation_import(path: &str) -> Result<Decision, CSTError> {
    ask(&format_confirm_import(path))
}
