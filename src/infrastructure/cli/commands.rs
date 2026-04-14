use crate::{
    infrastructure::cli::{
        interactor::{
            ask_confirmation_clear, ask_confirmation_import, ask_confirmation_remove,
            ask_confirmation_remove_many, ask_confirmation_restore,
        },
        ui::messages::confirm::Decision,
    },
    prelude::{CSTError, Filter, ImportMode, Priority, Sort, Status},
};

/// All commands the CLI can dispatch.
#[derive(Debug)]
pub enum TaskCommand {
    Add {
        information: String,
        priority: Option<Priority>,
        status: Option<Status>,
    },
    Get {
        id: i64,
    },
    List(Sort),
    Paged(Sort, i64),
    Done {
        id: i64,
    },
    DoneMany {
        ids: Vec<i64>,
    },
    Update {
        id: i64,
        information: Option<String>,
        priority: Option<Priority>,
        status: Option<Status>,
    },
    Remove {
        id: i64,
        confirmed: Option<Decision>,
    },
    Clear {
        confirmed: Option<Decision>,
    },
    Filter(Filter),
    Help,
    RemoveMany {
        ids: Vec<i64>,
        confirmed: Option<Decision>,
    },
    UpdateMany {
        ids: Vec<i64>,
        priority: Option<Priority>,
        status: Option<Status>,
    },
    Config(ConfigCommand),
    Import {
        path: String,
        mode: ImportMode,
        confirmed: Option<Decision>,
    },
    ExportCSV {
        output: Option<String>,
    },
    ExportMarkdown {
        output: Option<String>,
    },
    ExportExcel {
        output: Option<String>,
    },
    ExportJson {
        output: Option<String>,
    },
    ExportYaml {
        output: Option<String>,
    },
    ExportToml {
        output: Option<String>,
    },
    ImportJson {
        path: String,
        mode: ImportMode,
        confirmed: Option<Decision>,
    },
    ImportYaml {
        path: String,
        mode: ImportMode,
        confirmed: Option<Decision>,
    },
    ImportToml {
        path: String,
        mode: ImportMode,
        confirmed: Option<Decision>,
    },
    Undo,
    Version,
}

/// Subcommands for the config command.
#[derive(Debug)]
pub enum ConfigCommand {
    SetLanguage(String),
    SetDB(String),
    Show,
}

impl TaskCommand {
    /// Prompts the user for confirmation on destructive commands that lack it.
    ///
    /// Commands that already carry a [`Decision`] or don't require one are
    /// returned unchanged.
    pub fn ensure_confirmation(self) -> Result<Self, CSTError> {
        match self {
            TaskCommand::Import {
                path,
                mode,
                confirmed: None,
            } => format_confirm_import(
                path,
                mode,
                ask_confirmation_import,
                ask_confirmation_restore,
                |path, mode, confirmed| TaskCommand::Import {
                    path,
                    mode,
                    confirmed,
                },
            ),

            TaskCommand::ImportJson {
                path,
                mode,
                confirmed: None,
            } => format_confirm_import(
                path,
                mode,
                ask_confirmation_import,
                ask_confirmation_restore,
                |path, mode, confirmed| TaskCommand::ImportJson {
                    path,
                    mode,
                    confirmed,
                },
            ),

            TaskCommand::ImportYaml {
                path,
                mode,
                confirmed: None,
            } => format_confirm_import(
                path,
                mode,
                ask_confirmation_import,
                ask_confirmation_restore,
                |path, mode, confirmed| TaskCommand::ImportYaml {
                    path,
                    mode,
                    confirmed,
                },
            ),

            TaskCommand::ImportToml {
                path,
                mode,
                confirmed: None,
            } => format_confirm_import(
                path,
                mode,
                ask_confirmation_import,
                ask_confirmation_restore,
                |path, mode, confirmed| TaskCommand::ImportToml {
                    path,
                    mode,
                    confirmed,
                },
            ),

            TaskCommand::Remove {
                id,
                confirmed: None,
            } => ask_confirmation_remove(id).map(|d| TaskCommand::Remove {
                id,
                confirmed: Some(d),
            }),

            TaskCommand::RemoveMany {
                ids,
                confirmed: None,
            } => ask_confirmation_remove_many(&ids).map(|d| TaskCommand::RemoveMany {
                ids,
                confirmed: Some(d),
            }),

            TaskCommand::Clear { confirmed: None } => {
                ask_confirmation_clear().map(|d| TaskCommand::Clear { confirmed: Some(d) })
            }

            _ => Ok(self),
        }
    }
}

/// Asks for confirmation based on the import mode and wraps the result in a command.
///
/// `DryRun` skips confirmation entirely.
fn format_confirm_import<F, F1: Fn(&str) -> Result<Decision, CSTError>>(
    path: String,
    mode: ImportMode,
    ask_append: F1,
    ask_restore: impl Fn(&str) -> Result<Decision, CSTError>,
    build: F,
) -> Result<TaskCommand, CSTError>
where
    F: Fn(String, ImportMode, Option<Decision>) -> TaskCommand,
{
    match mode {
        ImportMode::Append => ask_append(&path).map(|d| build(path, ImportMode::Append, Some(d))),
        ImportMode::Restore => {
            ask_restore(&path).map(|d| build(path, ImportMode::Restore, Some(d)))
        }
        ImportMode::DryRun => Ok(build(path, ImportMode::DryRun, None)),
    }
}
