use crate::prelude::{
    Decision, Err, Filter, Mode, Priority, Sort, Status, prompt_clear, prompt_import,
    prompt_remove, prompt_remove_many, prompt_restore,
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
        mode: Mode,
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
        mode: Mode,
        confirmed: Option<Decision>,
    },
    ImportYaml {
        path: String,
        mode: Mode,
        confirmed: Option<Decision>,
    },
    ImportToml {
        path: String,
        mode: Mode,
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
    pub fn ensure_confirmation(self) -> Result<Self, Err> {
        match self {
            TaskCommand::Import {
                path,
                mode,
                confirmed: None,
            } => confirm_import(
                path,
                mode,
                prompt_import,
                prompt_restore,
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
            } => confirm_import(
                path,
                mode,
                prompt_import,
                prompt_restore,
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
            } => confirm_import(
                path,
                mode,
                prompt_import,
                prompt_restore,
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
            } => confirm_import(
                path,
                mode,
                prompt_import,
                prompt_restore,
                |path, mode, confirmed| TaskCommand::ImportToml {
                    path,
                    mode,
                    confirmed,
                },
            ),

            TaskCommand::Remove {
                id,
                confirmed: None,
            } => prompt_remove(id).map(|d| TaskCommand::Remove {
                id,
                confirmed: Some(d),
            }),

            TaskCommand::RemoveMany {
                ids,
                confirmed: None,
            } => prompt_remove_many(&ids).map(|d| TaskCommand::RemoveMany {
                ids,
                confirmed: Some(d),
            }),

            TaskCommand::Clear { confirmed: None } => {
                prompt_clear().map(|d| TaskCommand::Clear { confirmed: Some(d) })
            }

            _ => Ok(self),
        }
    }
}

/// Asks for confirmation based on the import mode and wraps the result in a command.
///
/// `DryRun` skips confirmation entirely.
fn confirm_import<F, F1: Fn(&str) -> Result<Decision, Err>>(
    path: String,
    mode: Mode,
    ask_append: F1,
    ask_restore: impl Fn(&str) -> Result<Decision, Err>,
    build: F,
) -> Result<TaskCommand, Err>
where
    F: Fn(String, Mode, Option<Decision>) -> TaskCommand,
{
    match mode {
        Mode::Append => ask_append(&path).map(|d| build(path, Mode::Append, Some(d))),
        Mode::Restore => ask_restore(&path).map(|d| build(path, Mode::Restore, Some(d))),
        Mode::DryRun => Ok(build(path, Mode::DryRun, None)),
    }
}
