use std::{
    io::{Write, stdin},
    path::Path,
};

use crate::prelude::{CSTError, CliErr, FileFormat, Settings};

/// Prompts the user for a filename and resolves it to an absolute path.
///
/// Appends the format's extension if not already present. Relative paths
/// are resolved against `base_dir`.
pub fn ask_filename(format: FileFormat, base_dir: &Path) -> Result<String, CSTError> {
    print!("{}", Settings::t("ui.filename"));
    std::io::stdout()
        .flush()
        .map_err(|_| CSTError::from(CliErr::StdoutFlushError))?;

    let name = stdin()
        .lines()
        .next()
        .ok_or(CSTError::from(CliErr::EmptyInputStream))?
        .map_err(|_| CSTError::from(CliErr::StdinReadError))?;

    let filename = if name.ends_with(format.extension()) {
        name
    } else {
        format!("{}.{}", name, format.extension())
    };

    if Path::new(&filename).is_absolute() {
        Ok(filename)
    } else {
        Ok(base_dir.join(filename).to_string_lossy().to_string())
    }
}
