/// Supported file formats for import and export.
pub enum FileFormat {
    Csv,
    Markdown,
    Excel,
    Json,
    Yaml,
    Toml,
}

impl FileFormat {
    /// Returns the file extension associated with this format.
    pub fn extension(&self) -> &'static str {
        match self {
            FileFormat::Csv => "csv",
            FileFormat::Markdown => "md",
            FileFormat::Excel => "xlsx",
            FileFormat::Json => "json",
            FileFormat::Yaml => "yml",
            FileFormat::Toml => "toml",
        }
    }
}
