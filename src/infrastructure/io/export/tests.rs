#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::prelude::{
        Builder, Priority, Status, Store, Task, from_csv, from_json, from_toml, to_csv, to_json,
        to_md, to_toml, to_yaml,
    };

    fn setup() {
        Store::init_for_tests();
    }

    fn make_tasks() -> Vec<Task> {
        vec![
            Builder::new()
                .id(1)
                .information("Crear API REST".to_string())
                .priority(Some(Priority::High))
                .status(Some(Status::Todo))
                .build_with_id()
                .unwrap(),
            Builder::new()
                .id(2)
                .information("Documentar endpoints".to_string())
                .priority(Some(Priority::Medium))
                .status(Some(Status::InProgress))
                .build_with_id()
                .unwrap(),
            Builder::new()
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
        let (count, out_path, _) = to_csv(&make_tasks(), Some(&path)).unwrap();
        assert_eq!(count, 3);
        assert!(std::path::Path::new(&out_path).exists());
    }

    #[test]
    fn export_csv_content_is_correct() {
        setup();
        let path = temp_path("output.csv");
        to_csv(&make_tasks(), Some(&path)).unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("information,priority,status"));
        assert!(content.contains("Crear API REST"));
        assert!(content.contains("high"));
        assert!(content.contains("todo"));
        assert!(content.contains("Documentar endpoints"));
        assert!(content.contains("in-progress"));
    }

    #[test]
    fn export_csv_empty_tasks() {
        setup();
        let path = temp_path("empty.csv");
        let (count, _, _) = to_csv(&[], Some(&path)).unwrap();
        assert_eq!(count, 0);
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("information,priority,status"));
    }

    // ── JSON ──────────────────────────────────────────────────────────────────

    #[test]
    fn export_json_creates_file() {
        setup();
        let path = temp_path("output.json");
        let (count, out_path, _) = to_json(&make_tasks(), Some(&path)).unwrap();
        assert_eq!(count, 3);
        assert!(std::path::Path::new(&out_path).exists());
    }

    #[test]
    fn export_json_content_is_valid_json() {
        setup();
        let path = temp_path("output.json");
        to_json(&make_tasks(), Some(&path)).unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert!(parsed.is_array());
        assert_eq!(parsed.as_array().unwrap().len(), 3);
    }

    #[test]
    fn export_json_content_has_correct_fields() {
        setup();
        let path = temp_path("output.json");
        to_json(&make_tasks(), Some(&path)).unwrap();
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
        let (count, _, _) = to_json(&[], Some(&path)).unwrap();
        assert_eq!(count, 0);
        let content = std::fs::read_to_string(&path).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert_eq!(parsed.as_array().unwrap().len(), 0);
    }

    // ── YAML ──────────────────────────────────────────────────────────────────

    #[test]
    fn export_yaml_creates_file() {
        setup();
        let path = temp_path("output.yml");
        let (count, out_path, _) = to_yaml(&make_tasks(), Some(&path)).unwrap();
        assert_eq!(count, 3);
        assert!(std::path::Path::new(&out_path).exists());
    }

    #[test]
    fn export_yaml_content_is_valid_yaml() {
        setup();
        let path = temp_path("output.yml");
        to_yaml(&make_tasks(), Some(&path)).unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        let parsed: serde_yaml::Value = serde_yaml::from_str(&content).unwrap();
        assert!(parsed.is_sequence());
        assert_eq!(parsed.as_sequence().unwrap().len(), 3);
    }

    #[test]
    fn export_yaml_content_has_correct_fields() {
        setup();
        let path = temp_path("output.yml");
        to_yaml(&make_tasks(), Some(&path)).unwrap();
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
        let (count, out_path, _) = to_toml(&make_tasks(), Some(&path)).unwrap();
        assert_eq!(count, 3);
        assert!(std::path::Path::new(&out_path).exists());
    }

    #[test]
    fn export_toml_content_is_valid_toml() {
        setup();
        let path = temp_path("output.toml");
        to_toml(&make_tasks(), Some(&path)).unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        let parsed: toml::Value = toml::from_str(&content).unwrap();
        let tasks = parsed["tasks"].as_array().unwrap();
        assert_eq!(tasks.len(), 3);
    }

    #[test]
    fn export_toml_content_has_correct_fields() {
        setup();
        let path = temp_path("output.toml");
        to_toml(&make_tasks(), Some(&path)).unwrap();
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
        let (count, out_path, _) = to_md(&make_tasks(), Some(&path)).unwrap();
        assert_eq!(count, 3);
        assert!(std::path::Path::new(&out_path).exists());
    }

    #[test]
    fn export_markdown_content_has_table_structure() {
        setup();
        let path = temp_path("output.md");
        to_md(&make_tasks(), Some(&path)).unwrap();
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
        let (count, _, _) = to_md(&[], Some(&path)).unwrap();
        assert_eq!(count, 0);
    }

    // ── Round-trip CSV ────────────────────────────────────────────────────────

    #[test]
    fn export_then_import_csv_roundtrip() {
        setup();
        let path = temp_path("roundtrip.csv");
        to_csv(&make_tasks(), Some(&path)).unwrap();
        let builders = from_csv(&path).unwrap();
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
        to_json(&make_tasks(), Some(&path)).unwrap();
        let builders = from_json(&path).unwrap();
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
        to_toml(&make_tasks(), Some(&path)).unwrap();
        let builders = from_toml(&path).unwrap();
        let imported: Vec<_> = builders.into_iter().map(|b| b.build().unwrap()).collect();
        assert_eq!(imported.len(), 3);
        assert_eq!(imported[0].information, "Crear API REST");
        assert_eq!(imported[0].priority, Priority::High);
        assert_eq!(imported[0].status, Status::Todo);
    }
}
