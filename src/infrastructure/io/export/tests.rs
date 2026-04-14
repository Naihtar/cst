#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        infrastructure::io::{
            export::{
                csv::export_csv, json::export_json, markdown::export_markdown, toml::export_toml,
                yaml::export_yaml,
            },
            import::{csv::import_csv, json::import_json, toml::import_toml},
        },
        prelude::{Priority, Settings, Status, Task, TaskBuilder},
    };

    fn setup() {
        Settings::init_for_tests();
    }

    fn make_tasks() -> Vec<Task> {
        vec![
            TaskBuilder::new()
                .id(1)
                .information("Crear API REST".to_string())
                .priority(Some(Priority::High))
                .status(Some(Status::Todo))
                .build_with_id()
                .unwrap(),
            TaskBuilder::new()
                .id(2)
                .information("Documentar endpoints.".to_string())
                .priority(Some(Priority::Medium))
                .status(Some(Status::InProgress))
                .build_with_id()
                .unwrap(),
            TaskBuilder::new()
                .id(3)
                .information("Testear base de datos".to_string())
                .priority(Some(Priority::Urgent))
                .status(Some(Status::Blocked))
                .build_with_id()
                .unwrap(),
        ]
    }

    fn temp_path(filename: &str) -> String {
        let dir: PathBuf = tempfile::tempdir().unwrap().keep();
        dir.join(filename).to_string_lossy().to_string()
    }

    // ── CSV ───────────────────────────────────────────────────────────────────

    #[test]
    fn export_csv_creates_file() {
        setup();
        let path = temp_path("output.csv");
        let (count, out_path, _) = export_csv(&make_tasks(), Some(&path)).unwrap();
        assert_eq!(count, 3);
        assert!(std::path::Path::new(&out_path).exists());
    }

    #[test]
    fn export_csv_content_is_correct() {
        setup();
        let path = temp_path("output.csv");
        export_csv(&make_tasks(), Some(&path)).unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("information,priority,status"));
        assert!(content.contains("Crear API REST"));
        assert!(content.contains("high"));
        assert!(content.contains("todo"));
        assert!(content.contains("Documentar endpoints."));
        assert!(content.contains("in-progress"));
    }

    #[test]
    fn export_csv_empty_tasks() {
        setup();
        let path = temp_path("empty.csv");
        let result = export_csv(&[], Some(&path));
        assert!(result.is_err());
    }

    // ── JSON ──────────────────────────────────────────────────────────────────

    #[test]
    fn export_json_creates_file() {
        setup();
        let path = temp_path("output.json");
        let (count, out_path, _) = export_json(&make_tasks(), Some(&path)).unwrap();
        assert_eq!(count, 3);
        assert!(std::path::Path::new(&out_path).exists());
    }

    #[test]
    fn export_json_content_is_valid_json() {
        setup();
        let path = temp_path("output.json");
        export_json(&make_tasks(), Some(&path)).unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert!(parsed.is_array());
        assert_eq!(parsed.as_array().unwrap().len(), 3);
    }

    #[test]
    fn export_json_content_has_correct_fields() {
        setup();
        let path = temp_path("output.json");
        export_json(&make_tasks(), Some(&path)).unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
        let first = &parsed[0];
        assert_eq!(first["information"], "Crear API REST");
        assert_eq!(first["priority"], "high");
        assert_eq!(first["status"], "todo");
    }

    #[test]
    fn export_json_empty_tasks() {
        setup();
        let path = temp_path("empty.json");
        let result = export_json(&[], Some(&path));
        assert!(result.is_err());
    }

    // ── YAML ──────────────────────────────────────────────────────────────────

    #[test]
    fn export_yaml_creates_file() {
        setup();
        let path = temp_path("output.yml");
        let (count, out_path, _) = export_yaml(&make_tasks(), Some(&path)).unwrap();
        assert_eq!(count, 3);
        assert!(std::path::Path::new(&out_path).exists());
    }

    #[test]
    fn export_yaml_content_is_valid_yaml() {
        setup();
        let path = temp_path("output.yml");
        export_yaml(&make_tasks(), Some(&path)).unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        let parsed: serde_yaml::Value = serde_yaml::from_str(&content).unwrap();
        assert!(parsed.is_sequence());
        assert_eq!(parsed.as_sequence().unwrap().len(), 3);
    }

    #[test]
    fn export_yaml_content_has_correct_fields() {
        setup();
        let path = temp_path("output.yml");
        export_yaml(&make_tasks(), Some(&path)).unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        let parsed: serde_yaml::Value = serde_yaml::from_str(&content).unwrap();
        let first = &parsed[0];
        assert_eq!(first["information"].as_str().unwrap(), "Crear API REST");
        assert_eq!(first["priority"].as_str().unwrap(), "high");
        assert_eq!(first["status"].as_str().unwrap(), "todo");
    }

    // ── TOML ──────────────────────────────────────────────────────────────────

    #[test]
    fn export_toml_creates_file() {
        setup();
        let path = temp_path("output.toml");
        let (count, out_path, _) = export_toml(&make_tasks(), Some(&path)).unwrap();
        assert_eq!(count, 3);
        assert!(std::path::Path::new(&out_path).exists());
    }

    #[test]
    fn export_toml_content_is_valid_toml() {
        setup();
        let path = temp_path("output.toml");
        export_toml(&make_tasks(), Some(&path)).unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        let parsed: toml::Value = toml::from_str(&content).unwrap();
        let tasks = parsed["tasks"].as_array().unwrap();
        assert_eq!(tasks.len(), 3);
    }

    #[test]
    fn export_toml_content_has_correct_fields() {
        setup();
        let path = temp_path("output.toml");
        export_toml(&make_tasks(), Some(&path)).unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        let parsed: toml::Value = toml::from_str(&content).unwrap();
        let first = &parsed["tasks"][0];
        assert_eq!(first["information"].as_str().unwrap(), "Crear API REST");
        assert_eq!(first["priority"].as_str().unwrap(), "high");
        assert_eq!(first["status"].as_str().unwrap(), "todo");
    }

    // ── Markdown ──────────────────────────────────────────────────────────────

    #[test]
    fn export_markdown_creates_file() {
        setup();
        let path = temp_path("output.md");
        let (count, out_path, _) = export_markdown(&make_tasks(), Some(&path)).unwrap();
        assert_eq!(count, 3);
        assert!(std::path::Path::new(&out_path).exists());
    }

    #[test]
    fn export_markdown_content_has_table_structure() {
        setup();
        let path = temp_path("output.md");
        export_markdown(&make_tasks(), Some(&path)).unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("| "));
        assert!(content.contains("Crear API REST"));
        assert!(content.contains("high"));
        assert!(content.contains("todo"));
    }

    #[test]
    fn export_markdown_empty_tasks() {
        setup();
        let path = temp_path("empty.md");
        let result = export_markdown(&[], Some(&path));
        assert!(result.is_err());
    }

    // ── Round-trip CSV ────────────────────────────────────────────────────────

    #[test]
    fn export_then_import_csv_roundtrip() {
        setup();
        let path = temp_path("roundtrip.csv");
        export_csv(&make_tasks(), Some(&path)).unwrap();
        let builders = import_csv(&path).unwrap();
        let imported: Vec<_> = builders.into_iter().map(|b| b.build().unwrap()).collect();
        assert_eq!(imported.len(), 3);
        assert_eq!(imported[0].information, "Crear API REST");
        assert_eq!(imported[0].priority, Priority::High);
        assert_eq!(imported[0].status, Status::Todo);
    }

    #[test]
    fn export_then_import_json_roundtrip() {
        setup();
        let path = temp_path("roundtrip.json");
        export_json(&make_tasks(), Some(&path)).unwrap();
        let builders = import_json(&path).unwrap();
        let imported: Vec<_> = builders.into_iter().map(|b| b.build().unwrap()).collect();
        assert_eq!(imported.len(), 3);
        assert_eq!(imported[0].information, "Crear API REST");
        assert_eq!(imported[0].priority, Priority::High);
        assert_eq!(imported[0].status, Status::Todo);
    }

    #[test]
    fn export_then_import_toml_roundtrip() {
        setup();
        let path = temp_path("roundtrip.toml");
        export_toml(&make_tasks(), Some(&path)).unwrap();
        let builders = import_toml(&path).unwrap();
        let imported: Vec<_> = builders.into_iter().map(|b| b.build().unwrap()).collect();
        assert_eq!(imported.len(), 3);
        assert_eq!(imported[0].information, "Crear API REST");
        assert_eq!(imported[0].priority, Priority::High);
        assert_eq!(imported[0].status, Status::Todo);
    }
}
