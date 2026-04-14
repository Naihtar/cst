#[cfg(test)]
mod tests {
    use crate::{
        config::settings::Settings,
        domain::models::task::{
            entity::NewTask,
            types::{Priority, Status},
        },
        infrastructure::io::import::{
            csv::import_csv, json::import_json, toml::import_toml, yaml::import_yaml,
        },
    };
    use std::path::PathBuf;

    fn setup() {
        Settings::init_for_tests();
    }

    fn fixture_path(filename: &str) -> String {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src/infrastructure/io/import/fixtures")
            .join(filename)
            .to_string_lossy()
            .to_string()
    }

    fn assert_fixture_tasks(tasks: &[NewTask]) {
        assert_eq!(tasks.len(), 4);

        assert_eq!(tasks[0].information, "Crear API REST");
        assert_eq!(tasks[0].priority, Priority::High);
        assert_eq!(tasks[0].status, Status::Todo);

        assert_eq!(tasks[1].information, "Documentar endpoints.");
        assert_eq!(tasks[1].priority, Priority::Medium);
        assert_eq!(tasks[1].status, Status::InProgress);

        assert_eq!(tasks[2].information, "Testear base de datos");
        assert_eq!(tasks[2].priority, Priority::Urgent);
        assert_eq!(tasks[2].status, Status::Blocked);

        assert_eq!(tasks[3].information, "Configurar backups");
        assert_eq!(tasks[3].priority, Priority::Low);
        assert_eq!(tasks[3].status, Status::Done);
    }

    // ── CSV ───────────────────────────────────────────────────────────────────

    #[test]
    fn import_csv_parses_correctly() {
        setup();
        let path = fixture_path("tasks.csv");
        let builders = import_csv(&path).unwrap();
        let tasks: Vec<_> = builders.into_iter().map(|b| b.build().unwrap()).collect();
        assert_fixture_tasks(&tasks);
    }

    #[test]
    fn import_csv_fails_with_missing_information_header() {
        setup();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("bad.csv");
        std::fs::write(&path, "name,priority,status\nTest,high,todo").unwrap();
        let result = import_csv(path.to_str().unwrap());
        assert!(result.is_err());
    }

    #[test]
    fn import_csv_fails_with_nonexistent_file() {
        setup();
        let result = import_csv("nonexistent.csv");
        assert!(result.is_err());
    }

    #[test]
    fn import_csv_uses_defaults_for_missing_priority_and_status() {
        setup();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("minimal.csv");
        std::fs::write(&path, "information,priority,status\nSolo información,,").unwrap();
        let builders = import_csv(path.to_str().unwrap()).unwrap();
        let task = builders.into_iter().next().unwrap().build().unwrap();
        assert_eq!(task.information, "Solo información");
        assert_eq!(task.priority, Priority::Low);
        assert_eq!(task.status, Status::Todo);
    }

    // ── JSON ──────────────────────────────────────────────────────────────────

    #[test]
    fn import_json_parses_correctly() {
        setup();
        let path = fixture_path("tasks.json");
        let builders = import_json(&path).unwrap();
        let tasks: Vec<_> = builders.into_iter().map(|b| b.build().unwrap()).collect();
        assert_fixture_tasks(&tasks);
    }

    #[test]
    fn import_json_fails_with_missing_information_field() {
        setup();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("bad.json");
        std::fs::write(&path, r#"[{"priority":"high","status":"todo"}]"#).unwrap();
        let result = import_json(path.to_str().unwrap());
        assert!(result.is_err());
    }

    #[test]
    fn import_json_fails_with_nonexistent_file() {
        setup();
        let result = import_json("nonexistent.json");
        assert!(result.is_err());
    }

    #[test]
    fn import_json_fails_with_invalid_json() {
        setup();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("invalid.json");
        std::fs::write(&path, "not json at all").unwrap();
        let result = import_json(path.to_str().unwrap());
        assert!(result.is_err());
    }

    // ── YAML ──────────────────────────────────────────────────────────────────

    #[test]
    fn import_yaml_parses_correctly() {
        setup();
        let path = fixture_path("tasks.yaml");
        let builders = import_yaml(&path).unwrap();
        let tasks: Vec<_> = builders.into_iter().map(|b| b.build().unwrap()).collect();
        assert_fixture_tasks(&tasks);
    }

    #[test]
    fn import_yaml_fails_with_missing_information_field() {
        setup();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("bad.yaml");
        std::fs::write(&path, "- priority: high\n  status: todo").unwrap();
        let result = import_yaml(path.to_str().unwrap());
        assert!(result.is_err());
    }

    #[test]
    fn import_yaml_fails_with_nonexistent_file() {
        setup();
        let result = import_yaml("nonexistent.yaml");
        assert!(result.is_err());
    }

    #[test]
    fn import_yaml_fails_with_invalid_yaml() {
        setup();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("invalid.yaml");
        std::fs::write(&path, ":\nno válido::\n").unwrap();
        let result = import_yaml(path.to_str().unwrap());
        assert!(result.is_err());
    }

    // ── TOML ──────────────────────────────────────────────────────────────────

    #[test]
    fn import_toml_parses_correctly() {
        setup();
        let path = fixture_path("tasks.toml");
        let builders = import_toml(&path).unwrap();
        let tasks: Vec<_> = builders.into_iter().map(|b| b.build().unwrap()).collect();
        assert_fixture_tasks(&tasks);
    }

    #[test]
    fn import_toml_fails_with_missing_information_field() {
        setup();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("bad.toml");
        std::fs::write(&path, "[[tasks]]\npriority = \"high\"\nstatus = \"todo\"").unwrap();
        let result = import_toml(path.to_str().unwrap());
        assert!(result.is_err());
    }

    #[test]
    fn import_toml_fails_with_nonexistent_file() {
        setup();
        let result = import_toml("nonexistent.toml");
        assert!(result.is_err());
    }

    #[test]
    fn import_toml_fails_with_invalid_toml() {
        setup();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("invalid.toml");
        std::fs::write(&path, "esto no es toml válido :::").unwrap();
        let result = import_toml(path.to_str().unwrap());
        assert!(result.is_err());
    }
}
