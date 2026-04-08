use crate::prelude::{ConfigErr, Err};

/// Supported UI languages.
pub enum Language {
    En,
    Es,
}

impl Language {
    /// Returns the BCP 47 language code as a static string slice.
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::En => "en",
            Language::Es => "es",
        }
    }
}

impl TryFrom<String> for Language {
    type Error = Err;

    /// Parses a language code string into a [`Language`] variant.
    ///
    /// Input is trimmed and lowercased before matching.
    /// Returns [`ConfigError::InvalidLanguage`] if the code is not supported.
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.trim().to_lowercase().as_str() {
            "en" => Ok(Language::En),
            "es" => Ok(Language::Es),
            _ => Err(ConfigErr::InvalidLanguage(s))?,
        }
    }
}
