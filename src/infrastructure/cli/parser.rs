use std::env;

use crate::{
    infrastructure::cli::{
        mappers::{
            accept_flag, char_to_config, extract_modifiers, modifiers_to_filter,
            modifiers_to_priority, modifiers_to_sort, modifiers_to_status, parse_id, parse_ids,
        },
        ui::messages::confirm::Decision,
    },
    prelude::{CSTError, CliErr, ConfigCommand, ImportMode, TaskCommand},
};

/// Parses CLI arguments from `env::args` into a [`TaskCommand`].
pub struct CliParser {
    args: Vec<String>,
}

impl Default for CliParser {
    /// Collects arguments from the environment, skipping the binary name.
    fn default() -> Self {
        Self {
            args: env::args().skip(1).collect(),
        }
    }
}

impl CliParser {
    pub fn new() -> Self {
        Self::default()
    }

    /// Parses the collected arguments into a [`TaskCommand`].
    ///
    /// Returns [`TaskCommand::Help`] if no arguments are provided.
    pub fn parse(&self) -> Result<TaskCommand, CSTError> {
        let Some(_) = self.args.first() else {
            return Ok(TaskCommand::Help);
        };

        match self.find_command_char()? {
            'A' => self.parse_add(),
            'G' => self.parse_get(),
            'L' => self.parse_list(),
            'P' => self.parse_paged(),
            'D' => self.parse_done(),
            'U' => self.parse_update(),
            'R' => self.parse_remove(),
            'F' => self.parse_filter(),
            'X' => self.parse_clear(),
            'C' => self.parse_config(),
            'I' => self.parse_import(),
            'J' => self.parse_export_with_flag(|output| TaskCommand::ExportJson { output }),
            'Y' => self.parse_export_with_flag(|output| TaskCommand::ExportYaml { output }),
            'T' => self.parse_export_with_flag(|output| TaskCommand::ExportToml { output }),
            'H' => Ok(TaskCommand::Help),
            'S' => Ok(TaskCommand::ExportCSV {
                output: self.positional_args().first().map(|s| s.to_string()),
            }),
            'M' => Ok(TaskCommand::ExportMarkdown {
                output: self.positional_args().first().map(|s| s.to_string()),
            }),
            'E' => Ok(TaskCommand::ExportExcel {
                output: self.positional_args().first().map(|s| s.to_string()),
            }),
            'Z' => Ok(TaskCommand::Undo),
            'V' => Ok(TaskCommand::Version),
            c => Err(CliErr::UnknownCommand(c))?,
        }
    }

    /// Finds the uppercase command letter in the flag arguments.
    fn find_command_char(&self) -> Result<char, CSTError> {
        Ok(self
            .args
            .iter()
            .filter(|arg| arg.starts_with('-'))
            .flat_map(|arg| arg.chars())
            .find(|c| c.is_ascii_uppercase())
            .ok_or(CliErr::MissingCommand)?)
    }

    /// Returns the flag argument that contains the uppercase command letter.
    fn find_command_flag(&self) -> Option<&str> {
        self.args
            .iter()
            .filter(|arg| arg.starts_with('-'))
            .find(|arg| arg.chars().any(|c| c.is_ascii_uppercase()))
            .map(|arg| arg.as_str())
    }

    /// Returns all arguments that do not start with `-`.
    fn positional_args(&self) -> Vec<&str> {
        self.args
            .iter()
            .filter(|arg| !arg.starts_with('-'))
            .map(|arg| arg.as_str())
            .collect()
    }

    /// Extracts a confirmation decision from the command flag modifiers, if present.
    fn find_confirmation(&self) -> Option<Decision> {
        self.find_command_flag()
            .map(extract_modifiers)
            .and_then(|m| m.iter().find_map(|&c| accept_flag(c)))
    }

    /// Extracts modifier characters from the command flag.
    fn modifiers(&self) -> Result<Vec<char>, CSTError> {
        Ok(self
            .find_command_flag()
            .map(extract_modifiers)
            .ok_or(CliErr::MissingCommand)?)
    }

    fn parse_add(&self) -> Result<TaskCommand, CSTError> {
        let modifiers = self.modifiers()?;
        let information = self
            .positional_args()
            .first()
            .ok_or(CliErr::MissingArgument('A'))?
            .to_string();

        Ok(TaskCommand::Add {
            information,
            priority: modifiers_to_priority(&modifiers),
            status: modifiers_to_status(&modifiers),
        })
    }

    fn parse_get(&self) -> Result<TaskCommand, CSTError> {
        let arg = self
            .positional_args()
            .first()
            .ok_or(CliErr::MissingArgument('G'))?
            .to_string();
        Ok(TaskCommand::Get {
            id: parse_id(&arg)?,
        })
    }

    fn parse_list(&self) -> Result<TaskCommand, CSTError> {
        let modifiers = self.modifiers()?;
        Ok(TaskCommand::List(modifiers_to_sort(&modifiers)))
    }

    fn parse_paged(&self) -> Result<TaskCommand, CSTError> {
        let modifiers = self.modifiers()?;
        let page = parse_page(&self.positional_args());
        Ok(TaskCommand::Paged(modifiers_to_sort(&modifiers), page))
    }

    fn parse_done(&self) -> Result<TaskCommand, CSTError> {
        let arg = self
            .positional_args()
            .first()
            .ok_or(CliErr::MissingArgument('D'))?
            .to_string();

        if arg.contains(',') {
            Ok(TaskCommand::DoneMany {
                ids: parse_ids(&arg)?,
            })
        } else {
            Ok(TaskCommand::Done {
                id: parse_id(&arg)?,
            })
        }
    }

    fn parse_remove(&self) -> Result<TaskCommand, CSTError> {
        let arg = self
            .positional_args()
            .first()
            .ok_or(CliErr::MissingArgument('R'))?
            .to_string();

        if arg.contains(',') {
            Ok(TaskCommand::RemoveMany {
                ids: parse_ids(&arg)?,
                confirmed: self.find_confirmation(),
            })
        } else {
            Ok(TaskCommand::Remove {
                id: parse_id(&arg)?,
                confirmed: self.find_confirmation(),
            })
        }
    }

    fn parse_update(&self) -> Result<TaskCommand, CSTError> {
        let modifiers = self.modifiers()?;
        let positional = self.positional_args();

        let (id_str, rest) = positional
            .split_first()
            .ok_or(CliErr::MissingArgument('U'))?;

        if id_str.contains(',') {
            Ok(TaskCommand::UpdateMany {
                ids: parse_ids(id_str)?,
                priority: modifiers_to_priority(&modifiers),
                status: modifiers_to_status(&modifiers),
            })
        } else {
            Ok(TaskCommand::Update {
                id: parse_id(id_str)?,
                information: rest.first().map(|s| s.to_string()),
                priority: modifiers_to_priority(&modifiers),
                status: modifiers_to_status(&modifiers),
            })
        }
    }

    fn parse_filter(&self) -> Result<TaskCommand, CSTError> {
        let modifiers = self.modifiers()?;
        let positional = self.positional_args();
        let word = parse_word(&positional);
        let page = parse_page(&positional);
        Ok(TaskCommand::Filter(modifiers_to_filter(
            &modifiers, word, page,
        )))
    }

    fn parse_clear(&self) -> Result<TaskCommand, CSTError> {
        Ok(TaskCommand::Clear {
            confirmed: self.find_confirmation(),
        })
    }

    fn parse_config(&self) -> Result<TaskCommand, CSTError> {
        let modifiers = self.modifiers()?;

        let config = modifiers
            .iter()
            .find_map(|&c| char_to_config(c))
            .ok_or(CliErr::MissingArgument('C'));

        match config {
            Err(_) => Ok(TaskCommand::Config(ConfigCommand::Show)),
            Ok(config) => {
                let value = self
                    .positional_args()
                    .first()
                    .ok_or(CliErr::MissingArgument('C'))?
                    .to_string();

                let command = match config {
                    ConfigCommand::SetLanguage(_) => ConfigCommand::SetLanguage(value),
                    ConfigCommand::SetDB(_) => ConfigCommand::SetDB(value),
                    ConfigCommand::Show => ConfigCommand::Show,
                };

                Ok(TaskCommand::Config(command))
            }
        }
    }

    /// Parses an import command, inferring the format from modifiers or file extension.
    fn parse_import(&self) -> Result<TaskCommand, CSTError> {
        let path = self
            .positional_args()
            .first()
            .ok_or(CliErr::MissingArgument('I'))?
            .to_string();

        let modifiers = self.modifiers().unwrap_or_default();

        let mode = if modifiers.contains(&'d') {
            ImportMode::DryRun
        } else if modifiers.contains(&'r') {
            ImportMode::Restore
        } else {
            ImportMode::Append
        };

        let confirmed = self.find_confirmation();

        if modifiers.contains(&'j') {
            return Ok(TaskCommand::ImportJson {
                path,
                mode,
                confirmed,
            });
        }
        if modifiers.contains(&'a') {
            return Ok(TaskCommand::ImportYaml {
                path,
                mode,
                confirmed,
            });
        }
        if modifiers.contains(&'t') {
            return Ok(TaskCommand::ImportToml {
                path,
                mode,
                confirmed,
            });
        }

        match path.rsplit('.').next().map(|e| e.to_lowercase()).as_deref() {
            Some("json") => Ok(TaskCommand::ImportJson {
                path,
                mode,
                confirmed,
            }),
            Some("yaml") | Some("yml") => Ok(TaskCommand::ImportYaml {
                path,
                mode,
                confirmed,
            }),
            Some("toml") => Ok(TaskCommand::ImportToml {
                path,
                mode,
                confirmed,
            }),
            _ => Ok(TaskCommand::Import {
                path,
                mode,
                confirmed,
            }),
        }
    }

    fn parse_export_with_flag<F>(&self, f: F) -> Result<TaskCommand, CSTError>
    where
        F: Fn(Option<String>) -> TaskCommand,
    {
        Ok(f(self.positional_args().first().map(|s| s.to_string())))
    }
}

/// Extracts the page number from a `@N` positional argument. Defaults to 0.
///
/// Input is 1-based; stored as 0-based internally.
fn parse_page(positional: &[&str]) -> i64 {
    positional
        .iter()
        .find(|s| s.starts_with('@'))
        .and_then(|s| s[1..].parse::<i64>().ok())
        .map(|p| (p - 1).max(0))
        .unwrap_or(0)
}

/// Returns the first positional argument that does not start with `@`, if any.
fn parse_word(positional: &[&str]) -> Option<String> {
    positional
        .iter()
        .find(|s| !s.starts_with('@'))
        .map(|s| s.to_string())
}
