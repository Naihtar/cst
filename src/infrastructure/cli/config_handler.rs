use crate::{
    infrastructure::cli::ui::messages::help::{format_config, format_help, format_version},
    prelude::{CSTError, CliOutput, ConfigCommand, Settings},
};

/// Returns the formatted help text.
pub fn handle_help() -> Result<CliOutput, CSTError> {
    Ok(CliOutput::Message(format_help()))
}

/// Returns the formatted version string.
pub fn handle_version() -> Result<CliOutput, CSTError> {
    Ok(CliOutput::Message(format_version()))
}

/// Handles config subcommands: show current settings or update DB path / language.
pub fn handle_config(command: ConfigCommand) -> Result<CliOutput, CSTError> {
    let settings = Settings::get()?;
    match command {
        ConfigCommand::Show => Ok(CliOutput::Message(format_config(settings))),
        ConfigCommand::SetLanguage(lang) => {
            Settings::save(&settings.db_path, &lang).map(|_| CliOutput::Success)
        }
        ConfigCommand::SetDB(db) => {
            Settings::save(&db, settings.language.as_str()).map(|_| CliOutput::Success)
        }
    }
}
