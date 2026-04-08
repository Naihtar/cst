use crate::prelude::{CfgCmd, Err, Output, Store, config_msg, help_msg, version_msg};

/// Returns the formatted help text.
pub fn handle_help() -> Result<Output, Err> {
    Ok(Output::Message(help_msg()))
}

/// Returns the formatted version string.
pub fn handle_version() -> Result<Output, Err> {
    Ok(Output::Message(version_msg()))
}

/// Handles config subcommands: show current settings or update DB path / language.
pub fn handle_config(command: CfgCmd) -> Result<Output, Err> {
    let settings = Store::get()?;
    match command {
        CfgCmd::Show => Ok(Output::Message(config_msg(settings))),
        CfgCmd::SetLanguage(lang) => Store::save(&settings.db_path, &lang).map(|_| Output::Success),
        CfgCmd::SetDB(db) => Store::save(&db, settings.language.as_str()).map(|_| Output::Success),
    }
}
