use rust_embed::RustEmbed;
use std::{collections::HashMap, path::PathBuf, sync::OnceLock};

use crate::prelude::{ConfigErr, Err, Lang};

/// Application-wide settings: database path, language, and translations.
pub struct Settings {
    pub db_path: String,
    pub language: Lang,
    translations: HashMap<String, String>,
}

/// Global singleton holding the loaded settings.
static SETTINGS: OnceLock<Settings> = OnceLock::new();

/// Embedded locale files bundled at compile time from the `locales/` folder.
#[derive(RustEmbed)]
#[folder = "locales/"]
struct Asset;

/// Returns the path to the `.env` config file inside the OS config directory.
///
/// Creates the `cst/` directory if it does not exist.
fn get_config_path() -> Result<PathBuf, Err> {
    let mut path = dirs::config_dir().ok_or(ConfigErr::SettingsNotInitialized)?;
    path.push("cst");
    if !path.exists() {
        std::fs::create_dir_all(&path)?;
    }
    path.push(".env");
    Ok(path)
}

impl Settings {
    /// Loads settings from the `.env` config file into the global singleton.
    ///
    /// If the config file does not exist, a default one is created with
    /// `tasks.db` as the database path and `en` as the language.
    pub fn load() -> Result<(), Err> {
        let config_file = get_config_path()?;
        if !config_file.exists() {
            let mut db_default = config_file.clone();
            db_default.set_file_name("tasks.db");
            Self::save(&db_default.to_string_lossy(), "en")?;
        }
        dotenvy::from_path_override(&config_file).ok();
        let db_path = std::env::var("DB_PATH").map_err(|_| ConfigErr::MissingEnvVar("DB_PATH"))?;
        let lang_str =
            std::env::var("LANGUAGE").map_err(|_| ConfigErr::MissingEnvVar("LANGUAGE"))?;
        let language = Lang::try_from(lang_str)?;
        let translations = load_translations(language.as_str())?;
        SETTINGS.get_or_init(|| Settings {
            db_path,
            language,
            translations,
        });
        Ok(())
    }

    /// Returns a reference to the global [`Settings`] singleton.
    ///
    /// Returns [`ConfigErr::SettingsNotInitialized`] if [`Settings::load`] has not been called.
    pub fn get() -> Result<&'static Settings, Err> {
        Ok(SETTINGS.get().ok_or(ConfigErr::SettingsNotInitialized)?)
    }

    /// Looks up a translation string by dot-separated key.
    ///
    /// Falls back to the key itself if no translation is found.
    pub fn t(key: &str) -> String {
        SETTINGS
            .get()
            .and_then(|s| s.translations.get(key))
            .cloned()
            .unwrap_or_else(|| key.to_string())
    }

    /// Persists the given `db_path` and `language` to the config file.
    ///
    /// Relative paths are resolved against the config directory.
    /// Returns an error if the language code is invalid.
    pub fn save(db_path: &str, language: &str) -> Result<(), Err> {
        Lang::try_from(language.to_string())?;
        let config_file = get_config_path()?;
        let final_db_path = match std::path::Path::new(db_path).is_absolute() {
            true => db_path.to_string(),
            false => {
                let mut p = config_file.clone();
                p.set_file_name(db_path);
                p.to_string_lossy().to_string()
            }
        };
        std::fs::write(
            &config_file,
            format!("DB_PATH={}\nLANGUAGE={}\n", final_db_path, language),
        )?;
        Ok(())
    }

    /// Initializes the singleton with in-memory defaults for use in tests.
    #[cfg(test)]
    pub fn init_for_tests() {
        let translations = load_translations("en").unwrap_or_default();
        SETTINGS.get_or_init(|| Settings {
            db_path: ":memory:".to_string(),
            language: Lang::En,
            translations,
        });
    }
}

/// Loads and parses the embedded YAML locale file for the given language code.
fn load_translations(lang: &str) -> Result<HashMap<String, String>, Err> {
    let file = Asset::get(&format!("{}.yml", lang)).ok_or(ConfigErr::MissingEnvVar("LANGUAGE"))?;
    let content = std::str::from_utf8(file.data.as_ref())
        .map_err(|e| ConfigErr::ParseError(e.to_string()))?;
    let yaml = serde_yaml::from_str::<serde_yaml::Value>(content)?;
    Ok(flatten_yaml(&yaml))
}

/// Flattens a YAML mapping into dot-separated key-value pairs.
///
/// Converts nested structures into a flat map where keys represents the path.
///
/// # Example
///
/// `{ ui: { title: "CST" } }` → `{ "ui.title": "CST" }`
pub fn flatten_yaml(value: &serde_yaml::Value) -> HashMap<String, String> {
    let mut result = HashMap::new();
    let mut stack = vec![(value, String::new())];

    while let Some((current, prefix)) = stack.pop() {
        match current {
            serde_yaml::Value::Mapping(map) => {
                for (k, v) in map {
                    if let Some(key) = k.as_str() {
                        let next_prefix = if prefix.is_empty() {
                            key.to_string()
                        } else {
                            format!("{}.{}", prefix, key)
                        };
                        stack.push((v, next_prefix));
                    }
                }
            }
            serde_yaml::Value::String(s) => {
                if !prefix.is_empty() {
                    result.insert(prefix, s.clone());
                }
            }
            serde_yaml::Value::Number(n) => {
                result.insert(prefix, n.to_string());
            }
            serde_yaml::Value::Bool(b) => {
                result.insert(prefix, b.to_string());
            }
            _ => {}
        }
    }

    result
}
