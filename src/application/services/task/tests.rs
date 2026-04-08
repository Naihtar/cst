#[cfg(test)]
mod tests {
    use crate::prelude::{Filter, Priority, Service, Sort, SqliteTR, Status, Store};

    fn setup() -> Service<SqliteTR> {
        Store::init_for_tests();
        let repo = SqliteTR::new(":memory:").unwrap();
        Service::new(repo)
    }

    // ── add_task ──────────────────────────────────────────────────────────────

    #[test]
    fn add_task_returns_valid_id() {
        let service = setup();
        let id = service
            .add_task("Test task".to_string(), None, None)
            .unwrap();
        assert!(id > 0);
    }

    #[test]
    fn add_task_with_priority_and_status() {
        let service = setup();
        let id = service
            .add_task(
                "Test task".to_string(),
                Some(Priority::Urgent),
                Some(Status::Blocked),
            )
            .unwrap();
        let task = service.find_task_by_id(id).unwrap().unwrap();
        assert_eq!(task.priority(), &Priority::Urgent);
        assert_eq!(task.status(), &Status::Blocked);
    }

    #[test]
    fn add_task_fails_with_empty_information() {
        let service = setup();
        let result = service.add_task("".to_string(), None, None);
        assert!(result.is_err());
    }

    // ── find_task_by_id ───────────────────────────────────────────────────────

    #[test]
    fn find_task_by_id_returns_correct_task() {
        let service = setup();
        let id = service
            .add_task("Test task".to_string(), Some(Priority::High), None)
            .unwrap();
        let task = service.find_task_by_id(id).unwrap().unwrap();
        assert_eq!(task.information(), "Test task");
        assert_eq!(task.priority(), &Priority::High);
    }

    #[test]
    fn find_task_by_id_returns_none_for_nonexistent() {
        let service = setup();
        let result = service.find_task_by_id(999).unwrap();
        assert!(result.is_none());
    }

    // ── list_tasks ────────────────────────────────────────────────────────────

    #[test]
    fn list_tasks_returns_all() {
        let service = setup();
        service.add_task("Task 1".to_string(), None, None).unwrap();
        service.add_task("Task 2".to_string(), None, None).unwrap();
        service.add_task("Task 3".to_string(), None, None).unwrap();
        let tasks = service.list_tasks(Sort::default()).unwrap();
        assert_eq!(tasks.len(), 3);
    }

    #[test]
    fn list_tasks_empty() {
        let service = setup();
        let tasks = service.list_tasks(Sort::default()).unwrap();
        assert!(tasks.is_empty());
    }

    // ── paged_tasks ───────────────────────────────────────────────────────────

    #[test]
    fn paged_tasks_returns_correct_count() {
        let service = setup();
        for i in 1..=25 {
            service.add_task(format!("Task {}", i), None, None).unwrap();
        }
        let (tasks, total) = service.paged_tasks(Sort::default(), 0).unwrap();
        assert_eq!(tasks.len(), 20);
        assert_eq!(total, 25);
    }

    // ── update_task ───────────────────────────────────────────────────────────

    #[test]
    fn update_task_modifies_fields() {
        let service = setup();
        let id = service
            .add_task("Original".to_string(), Some(Priority::Low), None)
            .unwrap();
        service
            .update_task(
                id,
                Some("Updated".to_string()),
                Some(Priority::Urgent),
                Some(Status::Done),
            )
            .unwrap();
        let task = service.find_task_by_id(id).unwrap().unwrap();
        assert_eq!(task.information(), "Updated");
        assert_eq!(task.priority(), &Priority::Urgent);
        assert_eq!(task.status(), &Status::Done);
    }

    #[test]
    fn update_task_fails_for_nonexistent_id() {
        let service = setup();
        let result = service.update_task(999, None, Some(Priority::High), None);
        assert!(result.is_err());
    }

    #[test]
    fn update_task_saves_snapshot() {
        let service = setup();
        let id = service
            .add_task("Original".to_string(), None, None)
            .unwrap();
        service
            .update_task(id, Some("Updated".to_string()), None, None)
            .unwrap();
        service.undo().unwrap();
        let task = service.find_task_by_id(id).unwrap().unwrap();
        assert_eq!(task.information(), "Original");
    }

    // ── remove_task ───────────────────────────────────────────────────────────

    #[test]
    fn remove_task_deletes_correctly() {
        let service = setup();
        let id = service.add_task("Task".to_string(), None, None).unwrap();
        service.remove_task(id).unwrap();
        assert!(service.find_task_by_id(id).unwrap().is_none());
    }

    #[test]
    fn remove_task_saves_snapshot() {
        let service = setup();
        let id = service.add_task("Task".to_string(), None, None).unwrap();
        service.remove_task(id).unwrap();
        service.undo().unwrap();
        assert!(service.find_task_by_id(id).unwrap().is_some());
    }

    // ── clear_all_tasks ───────────────────────────────────────────────────────

    #[test]
    fn clear_all_tasks_removes_everything() {
        let service = setup();
        service.add_task("Task 1".to_string(), None, None).unwrap();
        service.add_task("Task 2".to_string(), None, None).unwrap();
        service.clear_all_tasks().unwrap();
        let tasks = service.list_tasks(Sort::default()).unwrap();
        assert!(tasks.is_empty());
    }

    // ── filter_tasks ──────────────────────────────────────────────────────────

    #[test]
    fn filter_tasks_by_word() {
        let service = setup();
        service
            .add_task("Crear API REST".to_string(), None, None)
            .unwrap();
        service
            .add_task("Documentar endpoints".to_string(), None, None)
            .unwrap();
        service
            .add_task("Testear API".to_string(), None, None)
            .unwrap();
        let (tasks, total) = service
            .filter_tasks(Filter {
                word: Some("API".to_string()),
                ..Filter::default()
            })
            .unwrap();
        assert_eq!(tasks.len(), 2);
        assert_eq!(total, 2);
    }

    // ── remove_many ───────────────────────────────────────────────────────────

    #[test]
    fn remove_many_returns_processed_and_missing() {
        let service = setup();
        let id1 = service.add_task("Task 1".to_string(), None, None).unwrap();
        let id2 = service.add_task("Task 2".to_string(), None, None).unwrap();
        let (processed, missing) = service.remove_many(vec![id1, id2, 999]).unwrap();
        assert_eq!(processed.len(), 2);
        assert_eq!(missing.len(), 1);
        assert!(missing.contains(&999));
    }

    // ── done_many ─────────────────────────────────────────────────────────────

    #[test]
    fn done_many_marks_tasks_as_done() {
        let service = setup();
        let id1 = service.add_task("Task 1".to_string(), None, None).unwrap();
        let id2 = service.add_task("Task 2".to_string(), None, None).unwrap();
        service.done_many(vec![id1, id2]).unwrap();
        assert_eq!(
            service.find_task_by_id(id1).unwrap().unwrap().status(),
            &Status::Done
        );
        assert_eq!(
            service.find_task_by_id(id2).unwrap().unwrap().status(),
            &Status::Done
        );
    }

    // ── update_many ───────────────────────────────────────────────────────────

    #[test]
    fn update_many_modifies_multiple_tasks() {
        let service = setup();
        let id1 = service.add_task("Task 1".to_string(), None, None).unwrap();
        let id2 = service.add_task("Task 2".to_string(), None, None).unwrap();
        let (processed, missing) = service
            .update_many(
                vec![id1, id2],
                Some(Priority::Urgent),
                Some(Status::Blocked),
            )
            .unwrap();
        assert_eq!(processed.len(), 2);
        assert!(missing.is_empty());
        assert_eq!(
            service.find_task_by_id(id1).unwrap().unwrap().priority(),
            &Priority::Urgent
        );
        assert_eq!(
            service.find_task_by_id(id2).unwrap().unwrap().status(),
            &Status::Blocked
        );
    }

    // ── undo ──────────────────────────────────────────────────────────────────

    #[test]
    fn undo_fails_without_snapshot() {
        let service = setup();
        let result = service.undo();
        assert!(result.is_err());
    }

    #[test]
    fn undo_restores_previous_state() {
        let service = setup();
        let id = service.add_task("Task".to_string(), None, None).unwrap();
        service.remove_task(id).unwrap();
        assert!(service.find_task_by_id(id).unwrap().is_none());
        service.undo().unwrap();
        assert!(service.find_task_by_id(id).unwrap().is_some());
    }

    // ── import preview ────────────────────────────────────────────────────────

    #[test]
    fn import_preview_returns_correct_count() {
        let service = setup();
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src/infrastructure/io/import/fixtures/tasks.csv")
            .to_string_lossy()
            .to_string();
        let count = service.import_preview(&path).unwrap();
        assert_eq!(count, 4);
    }
}
