use std::env;

use crate::prelude::{
    CfgCmd, CliErr, Cmd, Decision, Err, Mode, accept_flag, get_modifiers, parse_id, parse_ids,
    to_config_char, to_filter, to_priority, to_sort, to_status,
};

/// Parses CLI arguments from `env::args` into a [`Cmd`].
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

    /// Parses the collected arguments into a [`Cmd`].
    ///
    /// Returns [`Cmd::Help`] if no arguments are provided.
    pub fn parse(&self) -> Result<Cmd, Err> {
        let Some(_) = self.args.first() else {
            return Ok(Cmd::Help);
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
            'J' => self.parse_export_with_flag(|output| Cmd::ExportJson { output }),
            'Y' => self.parse_export_with_flag(|output| Cmd::ExportYaml { output }),
            'T' => self.parse_export_with_flag(|output| Cmd::ExportToml { output }),
            'H' => Ok(Cmd::Help),
            'S' => Ok(Cmd::ExportCSV {
                output: self.positional_args().first().map(|s| s.to_string()),
            }),
            'M' => Ok(Cmd::ExportMarkdown {
                output: self.positional_args().first().map(|s| s.to_string()),
            }),
            'E' => Ok(Cmd::ExportExcel {
                output: self.positional_args().first().map(|s| s.to_string()),
            }),
            'Z' => Ok(Cmd::Undo),
            'V' => Ok(Cmd::Version),
            c => Err(CliErr::UnknownCommand(c))?,
        }
    }

    /// Finds the uppercase command letter in the flag arguments.
    fn find_command_char(&self) -> Result<char, Err> {
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
            .map(get_modifiers)
            .and_then(|m| m.iter().find_map(|&c| accept_flag(c)))
    }

    /// Extracts modifier characters from the command flag.
    fn modifiers(&self) -> Result<Vec<char>, Err> {
        Ok(self
            .find_command_flag()
            .map(get_modifiers)
            .ok_or(CliErr::MissingCommand)?)
    }

    fn parse_add(&self) -> Result<Cmd, Err> {
        let modifiers = self.modifiers()?;
        let information = self
            .positional_args()
            .first()
            .ok_or(CliErr::MissingArgument('A'))?
            .to_string();

        Ok(Cmd::Add {
            information,
            priority: to_priority(&modifiers),
            status: to_status(&modifiers),
        })
    }

    fn parse_get(&self) -> Result<Cmd, Err> {
        let arg = self
            .positional_args()
            .first()
            .ok_or(CliErr::MissingArgument('G'))?
            .to_string();
        Ok(Cmd::Get {
            id: parse_id(&arg)?,
        })
    }

    fn parse_list(&self) -> Result<Cmd, Err> {
        let modifiers = self.modifiers()?;
        Ok(Cmd::List(to_sort(&modifiers)))
    }

    fn parse_paged(&self) -> Result<Cmd, Err> {
        let modifiers = self.modifiers()?;
        let page = parse_page(&self.positional_args());
        Ok(Cmd::Paged(to_sort(&modifiers), page))
    }

    fn parse_done(&self) -> Result<Cmd, Err> {
        let arg = self
            .positional_args()
            .first()
            .ok_or(CliErr::MissingArgument('D'))?
            .to_string();

        if arg.contains(',') {
            Ok(Cmd::DoneMany {
                ids: parse_ids(&arg)?,
            })
        } else {
            Ok(Cmd::Done {
                id: parse_id(&arg)?,
            })
        }
    }

    fn parse_remove(&self) -> Result<Cmd, Err> {
        let arg = self
            .positional_args()
            .first()
            .ok_or(CliErr::MissingArgument('R'))?
            .to_string();

        if arg.contains(',') {
            Ok(Cmd::RemoveMany {
                ids: parse_ids(&arg)?,
                confirmed: self.find_confirmation(),
            })
        } else {
            Ok(Cmd::Remove {
                id: parse_id(&arg)?,
                confirmed: self.find_confirmation(),
            })
        }
    }

    fn parse_update(&self) -> Result<Cmd, Err> {
        let modifiers = self.modifiers()?;
        let positional = self.positional_args();

        let (id_str, rest) = positional
            .split_first()
            .ok_or(CliErr::MissingArgument('U'))?;

        if id_str.contains(',') {
            Ok(Cmd::UpdateMany {
                ids: parse_ids(id_str)?,
                priority: to_priority(&modifiers),
                status: to_status(&modifiers),
            })
        } else {
            Ok(Cmd::Update {
                id: parse_id(id_str)?,
                information: rest.first().map(|s| s.to_string()),
                priority: to_priority(&modifiers),
                status: to_status(&modifiers),
            })
        }
    }

    fn parse_filter(&self) -> Result<Cmd, Err> {
        let modifiers = self.modifiers()?;
        let positional = self.positional_args();
        let word = parse_word(&positional);
        let page = parse_page(&positional);
        Ok(Cmd::Filter(to_filter(&modifiers, word, page)))
    }

    fn parse_clear(&self) -> Result<Cmd, Err> {
        Ok(Cmd::Clear {
            confirmed: self.find_confirmation(),
        })
    }

    fn parse_config(&self) -> Result<Cmd, Err> {
        let modifiers = self.modifiers()?;

        let config = modifiers
            .iter()
            .find_map(|&c| to_config_char(c))
            .ok_or(CliErr::MissingArgument('C'));

        match config {
            Err(_) => Ok(Cmd::Config(CfgCmd::Show)),
            Ok(config) => {
                let value = self
                    .positional_args()
                    .first()
                    .ok_or(CliErr::MissingArgument('C'))?
                    .to_string();

                let command = match config {
                    CfgCmd::SetLanguage(_) => CfgCmd::SetLanguage(value),
                    CfgCmd::SetDB(_) => CfgCmd::SetDB(value),
                    CfgCmd::Show => CfgCmd::Show,
                };

                Ok(Cmd::Config(command))
            }
        }
    }

    /// Parses an import command, inferring the format from modifiers or file extension.
    fn parse_import(&self) -> Result<Cmd, Err> {
        let path = self
            .positional_args()
            .first()
            .ok_or(CliErr::MissingArgument('I'))?
            .to_string();

        let modifiers = self.modifiers().unwrap_or_default();

        let mode = if modifiers.contains(&'d') {
            Mode::DryRun
        } else if modifiers.contains(&'r') {
            Mode::Restore
        } else {
            Mode::Append
        };

        let confirmed = self.find_confirmation();

        if modifiers.contains(&'j') {
            return Ok(Cmd::ImportJson {
                path,
                mode,
                confirmed,
            });
        }
        if modifiers.contains(&'a') {
            return Ok(Cmd::ImportYaml {
                path,
                mode,
                confirmed,
            });
        }
        if modifiers.contains(&'t') {
            return Ok(Cmd::ImportToml {
                path,
                mode,
                confirmed,
            });
        }

        match path.rsplit('.').next().map(|e| e.to_lowercase()).as_deref() {
            Some("json") => Ok(Cmd::ImportJson {
                path,
                mode,
                confirmed,
            }),
            Some("yaml") | Some("yml") => Ok(Cmd::ImportYaml {
                path,
                mode,
                confirmed,
            }),
            Some("toml") => Ok(Cmd::ImportToml {
                path,
                mode,
                confirmed,
            }),
            _ => Ok(Cmd::Import {
                path,
                mode,
                confirmed,
            }),
        }
    }

    fn parse_export_with_flag<F>(&self, f: F) -> Result<Cmd, Err>
    where
        F: Fn(Option<String>) -> Cmd,
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
