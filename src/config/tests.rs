#[cfg(test)]
mod language_tests {
    use crate::config::language::Language;

    #[test]
    fn as_str_en() {
        assert_eq!(Language::En.as_str(), "en");
    }

    #[test]
    fn as_str_es() {
        assert_eq!(Language::Es.as_str(), "es");
    }

    #[test]
    fn try_from_valid_en() {
        assert!(matches!(
            Language::try_from("en".to_string()),
            Ok(Language::En)
        ));
    }

    #[test]
    fn try_from_valid_es() {
        assert!(matches!(
            Language::try_from("es".to_string()),
            Ok(Language::Es)
        ));
    }

    #[test]
    fn try_from_uppercase_is_accepted() {
        assert!(matches!(
            Language::try_from("EN".to_string()),
            Ok(Language::En)
        ));
    }

    #[test]
    fn try_from_whitespace_is_trimmed() {
        assert!(matches!(
            Language::try_from("  es  ".to_string()),
            Ok(Language::Es)
        ));
    }

    #[test]
    fn try_from_invalid_returns_error() {
        let result = Language::try_from("fr".to_string());
        assert!(matches!(
            result,
            Err(e) if e.to_string().contains("fr")
        ));
    }

    #[test]
    fn try_from_empty_returns_error() {
        assert!(Language::try_from("".to_string()).is_err());
    }
}

#[cfg(test)]
mod settings_tests {
    use crate::config::settings::{Settings, flatten_yaml};

    #[test]
    fn flatten_yaml_nested() {
        let yaml: serde_yaml::Value =
            serde_yaml::from_str("ui:\n  title: CST\n  footer: v1").unwrap();
        let flat = flatten_yaml(&yaml);
        assert_eq!(flat.get("ui.title").map(String::as_str), Some("CST"));
        assert_eq!(flat.get("ui.footer").map(String::as_str), Some("v1"));
    }

    #[test]
    fn flatten_yaml_top_level_string() {
        let yaml: serde_yaml::Value = serde_yaml::from_str("key: value").unwrap();
        let flat = flatten_yaml(&yaml);
        assert_eq!(flat.get("key").map(String::as_str), Some("value"));
    }

    #[test]
    fn flatten_yaml_number_and_bool() {
        let yaml: serde_yaml::Value = serde_yaml::from_str("count: 42\nactive: true").unwrap();
        let flat = flatten_yaml(&yaml);
        assert_eq!(flat.get("count").map(String::as_str), Some("42"));
        assert_eq!(flat.get("active").map(String::as_str), Some("true"));
    }

    #[test]
    fn flatten_yaml_empty_mapping() {
        let yaml: serde_yaml::Value = serde_yaml::from_str("{}").unwrap();
        let flat = flatten_yaml(&yaml);
        assert!(flat.is_empty());
    }

    #[test]
    fn settings_t_fallback_to_key() {
        let result = Settings::t("nonexistent.key");
        assert_eq!(result, "nonexistent.key");
    }

    #[test]
    fn init_for_tests_makes_get_succeed() {
        Settings::init_for_tests();
        assert!(Settings::get().is_ok());
    }

    #[test]
    fn init_for_tests_db_path_is_memory() {
        Settings::init_for_tests();
        let s = Settings::get().unwrap();
        assert_eq!(s.db_path, ":memory:");
    }

    #[test]
    fn init_for_tests_language_is_en() {
        Settings::init_for_tests();
        let s = Settings::get().unwrap();
        assert_eq!(s.language.as_str(), "en");
    }
}
